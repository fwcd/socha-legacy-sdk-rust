//! The game structures for the "Hive" game.

use arrayvec::ArrayVec;
use itertools::Itertools;
use lazy_static::lazy_static;
use log::{trace, debug};
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::TryFrom;
use std::cmp::{max, min};
use std::str::FromStr;
use std::fmt;
use socha_client_base::util::{SCResult, HasOpponent};
use socha_client_base::error::SCError;
use socha_client_base::xml_node::{FromXmlNode, XmlNode, XmlNodeBuilder};
use crate::util::{AxialCoords, CubeCoords, DoubledCoords, LineFormable, Adjacentable};

// Structures

/// A player color in the game.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PlayerColor {
	Red,
	Blue
}

/// Metadata about a player.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Player {
	pub color: PlayerColor,
	pub display_name: String
}

/// A snapshot of the game's state at
/// a specific turn. Consists of the
/// board and information about both players.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
	pub turn: u32,
	pub start_player_color: PlayerColor,
	pub current_player_color: PlayerColor,
	pub board: Board,
	red_player: Player,
	blue_player: Player,
	undeployed_red_pieces: Vec<Piece>,
	undeployed_blue_pieces: Vec<Piece>
}

/// The game board which is a symmetric hex grid with
/// a side length of 6 fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
	// TODO: Store fields contiguously in a Vec
	// and convert between coords and indices
	fields: HashMap<AxialCoords, Field>
}

/// A field on the game board.
/// 
/// Note that the field structure intentionally does _not_
/// store a position. If this is desired, you should use
/// `PositionedField` or a tuple, depending on whether you
/// want to express ownership over the field.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Field {
	piece_stack: Vec<Piece>,
	is_obstructed: bool
}

/// An owned field and a position.
/// 
/// If ownership over the field is not desired, you should
/// use a tuple instead.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PositionedField<C=AxialCoords> {
	pub field: Field,
	pub coords: C
}

/// A transition between two game states.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Move<C=AxialCoords> {
	SetMove { piece: Piece, destination: PositionedField<C> },
	DragMove { start: PositionedField<C>, destination: PositionedField<C> }
}

/// A game piece.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Piece {
	pub owner: PlayerColor,
	pub piece_type: PieceType
}

/// A game piece type.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PieceType {
	Ant,
	Bee,
	Beetle,
	Grasshopper,
	Spider
}

// Constants

pub const ROUND_LIMIT: usize = 30;
pub const BOARD_RADIUS: usize = 6;
pub const FIELD_COUNT: usize = 91; // def count(radius): 1 if (radius == 1) else (radius - 1) * 6 + count(radius - 1)
pub const INITIAL_PIECE_TYPES: [PieceType; 11] = [
	PieceType::Bee,
	PieceType::Spider,
	PieceType::Spider,
	PieceType::Spider,
	PieceType::Grasshopper,
	PieceType::Grasshopper,
	PieceType::Beetle,
	PieceType::Beetle,
	PieceType::Ant,
	PieceType::Ant,
	PieceType::Ant
];

// General implementations

impl Field {
	/// Creates a new field.
	pub fn new(piece_stack: impl IntoIterator<Item=Piece>, is_obstructed: bool) -> Self {
		Self { piece_stack: piece_stack.into_iter().collect(), is_obstructed: is_obstructed }
	}

	/// Fetches the player color "owning" the field.
	pub fn owner(&self) -> Option<PlayerColor> { self.piece().map(|p| p.owner) }
	
	/// Tests whether the field is owned by the given owner.
	#[inline]
	pub fn is_owned_by(&self, color: PlayerColor) -> bool { self.owner() == Some(color) }
	
	/// Tests whether the field is occupied.
	#[inline]
	pub fn is_occupied(&self) -> bool { self.is_obstructed || self.has_pieces() }
	
	/// Tests whether the field is not occupied.
	#[inline]
	pub fn is_empty(&self) -> bool { !self.is_occupied() }
	
	/// Fetches the top-most piece.
	#[inline]
	pub fn piece(&self) -> Option<Piece> { self.piece_stack.last().cloned() }
	
	/// Tests whether the field contains pieces.
	#[inline]
	pub fn has_pieces(&self) -> bool { !self.piece_stack.is_empty() }
	
	/// Fetches the piece stack.
	#[inline]
	pub fn piece_stack(&self) -> &Vec<Piece> { &self.piece_stack }
	
	/// Pushes a piece onto the piece stack.
	#[inline]
	pub fn push(&mut self, piece: Piece) { self.piece_stack.push(piece) }
	
	/// Pops a piece from the piece stack or
	/// returns `None` if the stack is empty.
	#[inline]
	pub fn pop(&mut self) -> Option<Piece> { self.piece_stack.pop() }
}

// Source: Partially translated from https://github.com/CAU-Kiel-Tech-Inf/socha/blob/8399e73673971427624a73ef42a1b023c69268ec/plugin/src/shared/sc/plugin2020/util/GameRuleLogic.kt
	
impl Board {
	/// Creates a new board with the given fields.
	pub fn new(fields: impl Into<HashMap<AxialCoords, Field>>) -> Self {
		Self { fields: fields.into() }
	}
	
	/// Creates a new hexagonal board. In addition to the provided
	/// fields, the board is padded with empty fields up to the
	/// given radius.
	pub fn filling_radius(radius: usize, fields: impl Into<HashMap<AxialCoords, Field>>) -> Self {
		let mut fields_mut: HashMap<_, _> = fields.into();
		trace!("Filling up board, occupied fields: {:?}", fields_mut.iter().filter(|(_, f)| f.is_occupied()).collect::<Vec<_>>());

		let outer = i32::try_from(radius).expect("Radius is too large to fit in a 32-bit (signed) int");
		let inner = outer - 1;
		let all_coords = ((-inner)..=inner)
			.flat_map(|y| (max(-(inner + y), -inner)..=min(inner - y, inner))
				.map(move |x| AxialCoords::new(x, y)));
		
		for coords in all_coords {
			if !fields_mut.contains_key(&coords) {
				fields_mut.insert(coords, Field::default());
				trace!("Filling up field at {}", coords);
			}
		}
		
		let board = Self { fields: fields_mut };
		trace!("Created board with occupied fields {:?}", board.occupied_fields().collect::<Vec<_>>());
		board
	}

	/// Parses a board from a plain text
	/// hex grid of the following format:
	///
	/// ```ignore 
	///     /\  /\      
	///    /  \/  \     
	///    |BR |   |    
	///   /\  /\  /\    
	///  /  \/  \/  \   
	///  |   |GB |   |  
	///  \  /\  /\  /   
	///   \/  \/  \/    
	///    |   |   |    
	///    \  /\  /     
	///     \/  \/      
	/// ```
	/// 
	/// The rows should be "indented" alternatingly
	/// with the first row indented as depicted
	/// by the example above and the board should
	/// have a perfectly centered field.
	/// 
	/// Each hex field may or may not contain
	/// a `Field` described by a two-character
	/// notation where the _first_ character
	/// denotes the owner color and the _second_
	/// character the piece type (more details
	/// can be found in `Field`'s `FromStr`
	/// implementation). Empty or invalid field contents
	/// are ignored.
	/// 
	/// Note that the format currently does
	/// not support stacked pieces or obstructed
	/// fields.
	/// 
	/// The fields will be returned in the format
	/// of axial coordinates with the origin being
	/// located in the center of the board. The x-axis
	/// points to the right and the y-axis diagonally
	/// to the top-left.
	pub fn from_ascii_hex_grid(grid: impl Into<String>) -> SCResult<Self> {
		let double_positioned: Vec<_> = grid.into().lines()
			.map(|l| l.trim())
			.skip_while(|l| l.is_empty())
			.skip(2)
			.step_by(3)
			.enumerate()
			.map(|(y, line)| (i32::try_from(y).unwrap(), line))
			.flat_map(|(y, line)| line
				.split("|")
				.filter(|frag| !frag.is_empty())
				.enumerate()
				.map(|(x, frag)| (i32::try_from(x).unwrap(), frag))
				.map(move |(x, frag)| (
					DoubledCoords::new((2 * x) + ((y + 1) % 2), y),
					Field::from_str(frag.trim()).unwrap_or_else(|e| {
						debug!("Could not parse {}: {:?}", frag, e);
						Field::default()
					})
				)))
			.collect();
		let center = DoubledCoords::new(
			double_positioned.iter().map(|(c, _)| c.x()).max().unwrap_or(0),
			double_positioned.iter().map(|(c, _)| c.y()).max().unwrap_or(0)
		) / 2;
		debug!("Determined center at {:?}", center);
		debug!("Parsed fields at {:?}", double_positioned.iter().map(|(c, _)| *c - center).collect::<Vec<_>>());
		let fields: HashMap<_, _> = double_positioned.into_iter()
			.map(|(c, f)| (AxialCoords::from(c - center), f))
			.collect();
		debug!("Fields: {:?}", fields);
		Ok(Board { fields: fields })
	}

	/// Fetches a reference to the field at the given
	/// coordinates. The coordinates can be of and type
	/// (e.g. axial/cube) as long as they are convertible
	/// to axial coordinates.
	#[inline]
	pub fn field(&self, coords: impl Into<AxialCoords>) -> Option<&Field> {
		self.fields.get(&coords.into())
	}
	
	/// Mutably borrows a field.
	pub fn field_mut(&mut self, coords: impl Into<AxialCoords>) -> Option<&mut Field> {
		self.fields.get_mut(&coords.into())
	}
	
	/// Tests whether a given position is occupied.
	pub fn is_occupied(&self, coords: impl Into<AxialCoords>) -> bool {
		self.field(coords).map(|f| f.is_occupied()).unwrap_or(true)
	}
	
	/// Fetches all fields owned by the given color.
	pub fn fields_owned_by(&self, color: PlayerColor) -> impl Iterator<Item=(AxialCoords, &Field)> {
		self.fields().filter(move |(_, f)| f.is_owned_by(color))
	}
	
	/// Fetches all empty fields.
	pub fn empty_fields(&self) -> impl Iterator<Item=(AxialCoords, &Field)> {
		self.fields().filter(|(_, f)| f.is_empty())
	}
	
	/// Fetches all occupied fields.
	pub fn occupied_fields(&self) -> impl Iterator<Item=(AxialCoords, &Field)> {
		self.fields().filter(|(_, f)| f.is_occupied())
	}
	
	/// Fetches empty fields connected to the swarm.
	pub fn swarm_boundary(&self) -> impl Iterator<Item=(AxialCoords, &Field)> {
		self.fields().filter(|(_, f)| f.is_occupied())
			.flat_map(move |(c, _)| self.empty_neighbors(c))
	}
	
	/// Fetches all fields.
	#[inline]
	pub fn fields(&self) -> impl Iterator<Item=(AxialCoords, &Field)> {
		self.fields.iter().map(|(&c, f)| (c, f))
	}
	
	/// Tests whether the board contains the given coordinate.
	#[inline]
	pub fn contains_coords(&self, coords: impl Into<AxialCoords>) -> bool {
		self.fields.contains_key(&coords.into())
	}
	
	/// Tests whether the board has any pieces.
	pub fn has_pieces(&self) -> bool {
		self.fields().any(|(_, f)| f.has_pieces())
	}
	
	/// Fetches the (existing) neighbor fields on the board.
	#[inline]
	pub fn neighbors<'a>(&'a self, coords: impl Into<AxialCoords>) -> impl Iterator<Item=(AxialCoords, &Field)> + 'a {
		coords.into().coord_neighbors().into_iter().filter_map(move |c| self.field(c).map(|f| (c, f)))
	}
	
	/// Fetches the unoccupied neighbor fields.
	pub fn empty_neighbors(&self, coords: impl Into<AxialCoords>) -> impl Iterator<Item=(AxialCoords, &Field)> {
		self.neighbors(coords).filter(|(_, f)| f.is_empty())
	}
	
	/// Tests whether the bee of the given color has been placed.
	pub fn has_placed_bee(&self, color: PlayerColor) -> bool {
		let bee = Piece { piece_type: PieceType::Bee, owner: color };
		self.fields().flat_map(|(_, f)| f.piece_stack()).any(|&p| p == bee)
	}
	
	/// Tests whether the field at the given coordinates is next to
	/// a given color.
	pub fn is_next_to(&self, color: PlayerColor, coords: impl Into<AxialCoords>) -> bool {
		self.neighbors(coords).any(|(_, f)| f.is_owned_by(color))
	}
	
	/// Tests whether the field at the given coordinates is adjacent
	/// to a field.
	pub fn is_next_to_piece(&self, coords: impl Into<AxialCoords>) -> bool {
		self.neighbors(coords).any(|(_, f)| f.has_pieces())
	}
	
	/// Fetches the possible destinations for a SetMove.
	fn possible_set_move_destinations<'a>(&'a self, color: PlayerColor) -> impl Iterator<Item=AxialCoords> + 'a {
		let opponent = color.opponent();

		trace!("Looking for SetMove destinations on board...");
		trace!("Fields owned by {:?}: {:#?}", color, self.fields_owned_by(color).collect::<Vec<_>>());
		trace!("Fields owned by {:?} (opponent): {:#?}", opponent, self.fields_owned_by(opponent).collect::<Vec<_>>());

		self.fields_owned_by(color)
			.flat_map(move |(c, _)| self.empty_neighbors(c))
			.unique()
			.filter_map(move |(c, _)| if self.is_next_to(opponent, c) { None } else {
				trace!("SetMove destination {} does not touch an opponent's ({:?}'s) piece, neighbors: {:#?}", c, opponent, self.neighbors(c).collect::<Vec<_>>());
				Some(c)
			})
	}
	
	/// Performs a depth-first search on the board's non-empty fields
	/// starting at the given coordinates and removing visited
	/// locations from the set.
	fn dfs_swarm(&self, coords: AxialCoords, unvisited: &mut HashSet<AxialCoords>) {
		if self.field(coords).filter(|f| f.has_pieces()).is_some() {
			unvisited.remove(&coords);
			for (neighbor, _) in self.neighbors(coords) {
				if unvisited.contains(&neighbor) {
					self.dfs_swarm(neighbor, unvisited)
				}
			}
		}
	}
	
	/// Tests whether a field satisfying the search condition can be
	/// reached by breadth-first searching the accessible fields.
	fn bfs_accessible(&self, start: AxialCoords, search_condition: impl Fn(AxialCoords, &Field) -> bool) -> bool {
		let mut queue = VecDeque::new();
		let mut visited = HashSet::new();
		queue.push_back(start);
		
		while let Some(coords) = queue.pop_front() {
			visited.insert(coords);

			if let Some(field) = self.field(coords) {
				if search_condition(coords, field) {
					return true;
				} else {
					queue.extend(self.accessible_neighbors(coords).filter_map(|(c, _)| if !visited.contains(&c) { Some(c) } else { None }));
				}
			}
		}

		false
	}
	
	/// Tests whether the given field can be reached in 3 moves
	/// by breadth-first searching the accessible fields.
	fn bfs_reachable_in_3_steps(&self, start: AxialCoords, destination: AxialCoords) -> bool {
		let mut paths_queue: VecDeque<ArrayVec<[AxialCoords; 3]>> = VecDeque::new();
		paths_queue.push_back({
			let mut path = ArrayVec::new();
			path.push(start);
			path
		});

		while let Some(path) = paths_queue.pop_front() {
			let mut neighbors = self.accessible_neighbors(path.last().cloned().unwrap()).filter(|(c, _)| !path.contains(c));
			if path.len() < 3 {
				paths_queue.extend(neighbors.map(|(c, _)| {
					let mut next_path = path.clone();
					next_path.push(c);
					next_path
				}));
			} else if neighbors.any(|(c, _)| c == destination) {
				return true;
			}
		}

		false
	}
	
	/// Finds the intersection between `a`'s and `b`'s neighbors.
	pub fn shared_neighbors(&self, a: impl Into<AxialCoords>, b: impl Into<AxialCoords>) -> Vec<(AxialCoords, &Field)> {
		let a_neighbors: HashSet<_> = self.neighbors(a).collect();
		let b_neighbors: HashSet<_> = self.neighbors(b).collect();
		a_neighbors.intersection(&b_neighbors).cloned().collect()
	}
	
	/// Tests whether a move between the given two
	/// locations is possible.
	pub fn can_move_between(&self, a: impl Into<AxialCoords>, b: impl Into<AxialCoords>) -> bool {
		let shared = self.shared_neighbors(a, b);
		(shared.len() == 1 || shared.iter().any(|(_, f)| f.is_empty())) && shared.iter().any(|(_, f)| f.has_pieces())
	}
	
	/// Finds the accessible neighbors.
	pub fn accessible_neighbors<'a>(&'a self, coords: impl Into<AxialCoords> + Copy + 'a) -> impl Iterator<Item=(AxialCoords, &Field)> + 'a {
		self.neighbors(coords).filter(move |(c, f)| f.is_empty() && self.can_move_between(coords, *c))
	}
	
	/// Tests whether two coordinates are connected by a path
	/// along the swarm's boundary.
	pub fn connected_by_boundary_path(&self, start_coords: impl Into<AxialCoords>, destination_coords: impl Into<AxialCoords>) -> bool {
		let start = start_coords.into();
		let destination = destination_coords.into();
		self.bfs_accessible(start, |c, _| c == destination)
	}
	
	/// Performs a depth-first search on the board at the given
	/// position to test whether the swarm is connected.
	pub fn is_swarm_connected(&self) -> bool {
		let mut unvisited = self.fields.iter()
			.filter_map(|(&c, f)| if f.has_pieces() { Some(c) } else { None })
			.collect::<HashSet<AxialCoords>>();

		if let Some(start) = unvisited.iter().next() {
			self.dfs_swarm(*start, &mut unvisited);
			unvisited.is_empty()
		} else {
			true // An empty swarm is connected
		}
	}
}

impl GameState {
	/// Fetches the undeployed pieces for a specific color.
	pub fn undeployed_pieces(&self, color: PlayerColor) -> &Vec<Piece> {
		match color {
			PlayerColor::Red => &self.undeployed_red_pieces,
			PlayerColor::Blue => &self.undeployed_blue_pieces
		}
	}
	
	/// Fetches the player data for a given color.
	pub fn player(&self, color: PlayerColor) -> &Player {
		match color {
			PlayerColor::Red => &self.red_player,
			PlayerColor::Blue => &self.blue_player
		}
	} 

	/// Fetches the current _round_ (which is half the turn).
	pub fn round(&self) -> u32 { self.turn / 2 }

	/// Ensures that the destination is a direct neighbor of the start.
	fn validate_adjacent(&self, start: AxialCoords, destination: AxialCoords) -> SCResult<()> {
		if start.is_adjacent_to(destination) { Ok(()) } else { Err("Coords are not adjacent to each other".into()) }
	}
	
	fn validate_ant_move(&self, start: AxialCoords, destination: AxialCoords) -> SCResult<()> {
		if self.board.connected_by_boundary_path(start, destination) { Ok(()) } else { Err("Could not find path for ant".into()) }
	}
	
	fn validate_bee_move(&self, start: AxialCoords, destination: AxialCoords) -> SCResult<()> {
		self.validate_adjacent(start, destination)?;
		if self.board.can_move_between(start, destination) { Ok(()) } else { Err(format!("Cannot move between {:?} and {:?}", start, destination).into()) }
	}
	
	fn validate_beetle_move(&self, start: AxialCoords, destination: AxialCoords) -> SCResult<()> {
		self.validate_adjacent(start, destination)?;
		if self.board.shared_neighbors(start, destination).iter().any(|(_, f)| f.has_pieces()) || self.board.field(destination).map(|f| f.has_pieces()).unwrap_or(false) {
			Ok(())
		} else {
			Err("Beetle has to move along swarm".into())
		}
	}
	
	fn validate_grasshopper_move(&self, start: AxialCoords, destination: AxialCoords) -> SCResult<()> {
		if !start.forms_line_with(destination) {
			Err("Grasshopper can only move along straight lines".into())
		} else if start.is_adjacent_to(destination) {
			Err("Grasshopper must not move to a neighbor".into())
		} else if start.line_iter(destination).map(|c| AxialCoords::from(c)).any(|c| self.board.field(c).map(|f| f.is_empty()).unwrap_or(false)) {
			Err("Grasshopper cannot move over empty fields".into())
		} else {
			Ok(())
		}
	}
	
	fn validate_spider_move(&self, start: AxialCoords, destination: AxialCoords) -> SCResult<()> {
		if self.board.bfs_reachable_in_3_steps(start, destination) { Ok(()) } else { Err("No 3-step path found for Spider move".into()) }
	}

	fn validate_set_move(&self, color: PlayerColor, piece: Piece, destination_coords: impl Into<AxialCoords>) -> SCResult<()> {
		let destination = destination_coords.into();
		if !self.board.contains_coords(destination) {
			Err(format!("Move destination is out of bounds: {:?}", destination).into())
		} else if self.board.field(destination).map(|f| f.is_obstructed).unwrap_or(true) {
			Err(format!("Move destination is obstructed: {:?}", destination).into())
		} else if !self.board.fields().any(|(_, f)| f.has_pieces()) {
			Ok(())
		} else if self.board.fields_owned_by(color).count() == 0 {
			if self.board.is_next_to(color.opponent(), destination) {
				Ok(())
			} else {
				Err("Piece has to be placed next to an opponent's piece".into())
			}
		} else if (self.round() == 3) && (!self.board.has_placed_bee(color)) && (piece.piece_type != PieceType::Bee) {
			Err("Bee has to be placed in the fourth round or earlier".into())
		} else if !self.undeployed_pieces(color).contains(&piece) {
			Err("Piece is not undeployed".into())
		} else if !self.board.neighbors(destination).any(|(_, f)| f.is_owned_by(color)) {
			Err("Piece is not placed next to an own piece".into())
		} else if self.board.neighbors(destination).any(|(_, f)| f.is_owned_by(color)) {
			Err("Piece must not be placed next to an opponent's piece".into())
		} else {
			Ok(())
		}
	}

	fn validate_drag_move(&self, color: PlayerColor, start_coords: impl Into<AxialCoords>, destination_coords: impl Into<AxialCoords>) -> SCResult<()> {
		let start = start_coords.into();
		let destination = destination_coords.into();
		if !self.board.has_placed_bee(color) {
			Err("Bee has to be placed before committing a drag move".into())
		} else if !self.board.contains_coords(start) {
			Err(format!("Move start is out of bounds: {:?}", start).into())
		} else if !self.board.contains_coords(destination) {
			Err(format!("Move destination is out of bounds: {:?}", destination).into())
		} else if let Some(dragged_piece) = self.board.field(start).and_then(|f| f.piece()) {
			if dragged_piece.owner != color {
				Err("Cannot move opponent's piece".into())
			} else if start == destination {
				Err("Cannot move when start == destination".into())
			} else if self.board.field(destination).and_then(|f| f.piece()).map(|p| p.piece_type == PieceType::Beetle).unwrap_or(false) {
				Err("Only beetles can climb other pieces".into())
			} else if {
				let mut without_piece = self.board.clone();
				without_piece.field_mut(start).ok_or_else(|| "Start field does not exist")?.pop();
				!without_piece.is_swarm_connected()
			} {
				Err("Drag move would disconnect the swarm".into())
			} else {
				match dragged_piece.piece_type {
					PieceType::Ant => self.validate_ant_move(start, destination),
					PieceType::Bee => self.validate_bee_move(start, destination),
					PieceType::Beetle => self.validate_beetle_move(start, destination),
					PieceType::Grasshopper => self.validate_grasshopper_move(start, destination),
					PieceType::Spider => self.validate_spider_move(start, destination)
				}
			}
		} else {
			Err("No piece to move".into())
		}
	}
	
	//// Tests whether the given move is valid.
	pub fn validate_move(&self, color: PlayerColor, game_move: &Move) -> SCResult<()> {
		match game_move {
			Move::SetMove { piece, destination } => self.validate_set_move(color, *piece, destination.coords),
			Move::DragMove { start, destination } => self.validate_drag_move(color, start.coords, destination.coords)
		}
	}
	
	/// Fetches a list of possible `SetMove`s.
	fn possible_set_moves(&self, color: PlayerColor) -> Vec<Move> {
		trace!("Finding possible SetMoves");

		let undeployed = self.undeployed_pieces(color);
		let opponent = color.opponent();
		let destination_coords: Vec<_> = if undeployed.len() == INITIAL_PIECE_TYPES.len() {
			// No pieces placed yet
			if self.undeployed_pieces(opponent).len() == INITIAL_PIECE_TYPES.len() {
				// First turn
				trace!("Finding SetMoves during first turn...");
				self.board.empty_fields().map(|(c, _)| c).collect()
			} else {
				// Second turn
				trace!("Finding SetMoves during second turn...");
				self.board.fields_owned_by(opponent).flat_map(|(c, _)| self.board.empty_neighbors(c)).map(|(c, _)| c).collect()
			}
		} else {
			trace!("Querying SetMove destinations...");
			self.board.possible_set_move_destinations(color).collect()
		};

		let destinations = destination_coords.into_iter()
			.filter_map(|c| self.board.field(c).map(|f| PositionedField { coords: c, field: f.clone() }));
		trace!("Found SetMove destinations at {:#?}", destinations);
		
		if !self.board.has_placed_bee(color) && self.turn > 5 {
			trace!("Player has not placed bee yet, therefore placing it is the only valid move.");
			destinations
				.map(|d| Move::SetMove {
					piece: Piece { piece_type: PieceType::Bee, owner: color },
					destination: d
				})
				.collect()
		} else {
			trace!("Creating set moves from {:?} x {:?}", destinations, undeployed);
			destinations
				.flat_map(|d| undeployed.iter().map(move |&p| Move::SetMove { piece: p, destination: d.clone() }))
				.collect()
		}
	}
	
	/// Returns the validated move.
	fn validated(&self, color: PlayerColor, game_move: Move) -> SCResult<Move> {
		self.validate_move(color, &game_move).map(|_| game_move)
	}
	
	/// Fetches a list of possible `DragMove`s.
	fn possible_drag_moves(&self, color: PlayerColor) -> Vec<Move> {
		trace!("Finding possible DragMoves");

		self.board.fields_owned_by(color).flat_map(|(start_coords, start_field)| {
			let mut targets: Vec<_> = self.board.swarm_boundary().collect();

			if start_field.piece().filter(|f| f.piece_type == PieceType::Beetle).is_some() {
				targets.extend(self.board.neighbors(start_coords));
			}
			
			trace!("Drag targets from {}: {:#?}", start_coords, targets);
			targets.into_iter()
				.filter_map(move |(c, f)| self.validated(color, Move::DragMove {
					start: PositionedField { coords: start_coords, field: start_field.clone() },
					destination: PositionedField { coords: c, field: f.clone() }
				}).ok())
		}).collect()
	}
	
	/// Fetches a list of possible moves for a given color.
	pub fn possible_moves(&self, color: PlayerColor) -> Vec<Move> {
		trace!("Finding possible moves for color {:?}", color);
		trace!("Current board state:\n{}", self.board);

		let mut moves = self.possible_set_moves(color);
		moves.extend(self.possible_drag_moves(color));
		moves
	}
}

impl HasOpponent for PlayerColor {
	fn opponent(self) -> Self {
		match self {
			Self::Red => Self::Blue,
			Self::Blue => Self::Red
		}
	}
}

// General conversions

impl FromStr for PlayerColor {
	type Err = SCError;

	fn from_str(raw: &str) -> SCResult<Self> {
		match raw.to_uppercase().as_str() {
			"RED" => Ok(Self::Red),
			"BLUE" => Ok(Self::Blue),
			_ => Err(format!("Did not recognize player color {}", raw).into())
		}
	}
}

impl TryFrom<char> for PlayerColor {
	type Error = SCError;

	fn try_from(c: char) -> SCResult<Self> {
		match c.to_uppercase().next() {
			Some('R') => Ok(Self::Red),
			Some('B') => Ok(Self::Blue),
			_ => Err(format!("Did not recognize player color {}", c).into())
		}
	}
}

impl From<PlayerColor> for char {
	fn from(color: PlayerColor) -> char {
		match color {
			PlayerColor::Red => 'R',
			PlayerColor::Blue => 'B'
		}
	}
}

impl From<PlayerColor> for String {
	fn from(color: PlayerColor) -> String {
		match color {
			PlayerColor::Red => "RED",
			PlayerColor::Blue => "BLUE"
		}.to_owned()
	}
}

impl FromStr for PieceType {
	type Err = SCError;
	
	fn from_str(raw: &str) -> SCResult<Self> {
		match raw.to_uppercase().as_str() {
			"ANT" => Ok(Self::Ant),
			"BEE" => Ok(Self::Bee),
			"BEETLE" => Ok(Self::Beetle),
			"GRASSHOPPER" => Ok(Self::Grasshopper),
			"SPIDER" => Ok(Self::Spider),
			_ => Err(format!("Did not recognize piece type {}", raw).into())
		}
	}
}

impl TryFrom<char> for PieceType {
	type Error = SCError;
	
	fn try_from(c: char) -> SCResult<Self> {
		match c.to_uppercase().next() {
			Some('A') => Ok(Self::Ant),
			Some('B') => Ok(Self::Bee),
			Some('T') => Ok(Self::Beetle),
			Some('G') => Ok(Self::Grasshopper),
			Some('S') => Ok(Self::Spider),
			_ => Err(format!("Did not recognize piece type {}", c).into())
		}
	}
}

impl From<PieceType> for char {
	fn from(piece_type: PieceType) -> char {
		match piece_type {
			PieceType::Ant => 'A',
			PieceType::Bee => 'B',
			PieceType::Beetle => 'T',
			PieceType::Grasshopper => 'G',
			PieceType::Spider => 'S'
		}
	}
}

impl From<PieceType> for String {
	fn from(piece_type: PieceType) -> String {
		match piece_type {
			PieceType::Ant => "ANT",
			PieceType::Bee => "BEE",
			PieceType::Beetle => "BEETLE",
			PieceType::Grasshopper => "GRASSHOPPER",
			PieceType::Spider => "SPIDER"
		}.to_owned()
	}
}

lazy_static! {
	/// The syntax used for fields when parsing
	/// ASCII hex grid fields.
	static ref FIELD_SYNTAX: Regex = Regex::new(r"^([A-Z])([A-Z])$").unwrap();
}

impl FromStr for Field {
	type Err = SCError;
	
	/// Converts a field in a two-character notation
	/// to a field. The first character denotes the
	/// player color and the second character describes the
	/// piece type.
	/// 
	/// Obstructed fields and piece stacks are not (yet)
	/// supported.
	fn from_str(raw: &str) -> SCResult<Self> {
		if raw.is_empty() {
			Ok(Self::default())
		} else {
			let groups = FIELD_SYNTAX.captures(raw).ok_or_else(|| SCError::from(format!("{} does not match field syntax {}", raw, FIELD_SYNTAX.as_str())))?;
			let owner = PlayerColor::try_from(groups[1].chars().next().unwrap())?;
			let piece_type = PieceType::try_from(groups[2].chars().next().unwrap())?;
			let piece = Piece { piece_type: piece_type, owner: owner };
			Ok(Self { piece_stack: vec![piece], is_obstructed: false })
		}
	}
}

impl fmt::Display for Field {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if let Some(piece) = self.piece() {
			write!(f, "{}{}", char::from(piece.owner), char::from(piece.piece_type))
		} else {
			write!(f, "[]")
		}
	}
}

impl fmt::Display for Board {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let min_x = self.fields().map(|(c, _)| c.x()).min().ok_or(fmt::Error)?;
		let min_y = self.fields().map(|(c, _)| c.y()).min().ok_or(fmt::Error)?;
		let max_x = self.fields().map(|(c, _)| c.x()).max().ok_or(fmt::Error)?;
		let max_y = self.fields().map(|(c, _)| c.y()).max().ok_or(fmt::Error)?;

		for y in min_y..=max_y {
			for x in min_x..=max_x {
				if let Some(field) = self.field(AxialCoords::new(-y, -x)) {
					write!(f, "{}", field)?;
				} else {
					write!(f, "00")?;
				}
			}
			writeln!(f)?;
		}

		Ok(())
	}
}

// XML conversions

impl FromXmlNode for GameState {
	fn from_node(node: &XmlNode) -> SCResult<Self> {
		Ok(Self {
			turn: node.attribute("turn")?.parse()?,
			start_player_color: node.attribute("startPlayerColor")?.parse()?,
			current_player_color: node.attribute("currentPlayerColor")?.parse()?,
			red_player: Player::from_node(node.child_by_name("red")?)?,
			blue_player: Player::from_node(node.child_by_name("blue")?)?,
			board: Board::from_node(node.child_by_name("board")?)?,
			undeployed_red_pieces: node.child_by_name("undeployedRedPieces")?.childs_by_name("piece").map(Piece::from_node).collect::<Result<_, _>>()?,
			undeployed_blue_pieces: node.child_by_name("undeployedBluePieces")?.childs_by_name("piece").map(Piece::from_node).collect::<Result<_, _>>()?
		})
	}
}

impl FromXmlNode for Player {
	fn from_node(node: &XmlNode) -> SCResult<Self> {
		Ok(Self {
			color: node.attribute("color")?.parse()?,
			display_name: node.attribute("displayName")?.to_owned()
		})
	}
}

impl FromXmlNode for Board {
	fn from_node(node: &XmlNode) -> SCResult<Self> {
		Ok(Self::filling_radius(6, node.childs_by_name("fields")
			.flat_map(|child| child.childs_by_name("field")
				.map(|f| Ok((
					CubeCoords::new(
						f.attribute("x")?.parse()?,
						f.attribute("y")?.parse()?,
						f.attribute("z")?.parse()?
					).into(),
					Field::from_node(f)?
				))))
			.collect::<SCResult<HashMap<AxialCoords, Field>>>()?
		))
	}
}

impl FromXmlNode for Field {
	fn from_node(node: &XmlNode) -> SCResult<Self> {
		Ok(Self {
			piece_stack: node.childs_by_name("piece").map(Piece::from_node).collect::<Result<_, _>>()?,
			is_obstructed: node.attribute("isObstructed")?.parse()?
		})
	}
}

impl FromXmlNode for Piece {
	fn from_node(node: &XmlNode) -> SCResult<Self> {
		Ok(Self {
			owner: node.attribute("owner")?.parse()?,
			piece_type: node.attribute("type")?.parse()?
		})
	}
}

impl From<Move> for XmlNode {
	fn from(game_move: Move) -> Self {
		match game_move {
			Move::SetMove { piece, destination } => XmlNode::new("data")
				.attribute("class", "setmove")
				.child(piece)
				.child(XmlNodeBuilder::from(destination).name("destination"))
				.build(),
			Move::DragMove { start, destination } => XmlNode::new("data")
				.attribute("class", "dragmove")
				.child(XmlNodeBuilder::from(start).name("start"))
				.child(XmlNodeBuilder::from(destination).name("destination"))
				.build()
		}
	}
}

impl<'a, C> From<PositionedField<C>> for XmlNodeBuilder<'a> where C: Into<CubeCoords> {
	fn from(field: PositionedField<C>) -> Self {
		let cube_coords = field.coords.into();
		XmlNodeBuilder::default()
			.attribute("class", "field")
			.attribute("x", cube_coords.x().to_string())
			.attribute("y", cube_coords.y().to_string())
			.attribute("z", cube_coords.z().to_string())
			.attribute("isObstructed", field.field.is_obstructed.to_string())
			.childs(field.field.piece_stack().iter().map(|&p| XmlNode::from(p)))
	}
}

impl From<Piece> for XmlNode {
	fn from(piece: Piece) -> Self {
		XmlNode::new("piece")
			.attribute("owner", piece.owner)
			.attribute("type", piece.piece_type)
			.build()
	}
}
