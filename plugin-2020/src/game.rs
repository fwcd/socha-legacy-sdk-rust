//! The game structures for the "Hive" game.

use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;
use std::ops::{Add, Sub, Mul};
use socha_client_base::util::{SCResult, HasOpponent};
use socha_client_base::hashmap;
use socha_client_base::error::SCError;
use socha_client_base::xml_node::{FromXmlNode, XmlNode};

/// A player color in the game.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PlayerColor {
	Red,
	Blue
}

/// Metadata about a player.
#[derive(Debug, Clone, PartialEq, Eq)]
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

/// Axial coordinates on the hex grid.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct AxialCoords {
	x: i32,
	y: i32
}

/// Cube coordinates on the hex grid.
/// These are used by the protocol internally.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct CubeCoords {
	x: i32,
	y: i32,
	z: i32
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
	piece_stack: Vec<Piece>,
	pub is_obstructed: bool
}

/// A transition between two game states.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Move<C=AxialCoords> {
	SetMove { piece: Piece, destination: C },
	DragMove { start: C, destination: C }
}

/// A game piece.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Piece {
	pub owner: PlayerColor,
	pub piece_type: PieceType
}

/// A game piece type.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PieceType {
	Ant,
	Bee,
	Beetle,
	Grasshopper,
	Spider
}

// General implementations

impl AxialCoords {
	/// Creates new axial coordinates.
	#[inline]
	pub fn new(x: i32, y: i32) -> Self { Self { x: x, y: y } }
	
	/// Fetches the x-coordinate
	#[inline]
	pub fn x(self) -> i32 { self.x }
	
	/// Fetches the y-coordinate
	#[inline]
	pub fn y(self) -> i32 { self.y }

	/// Fetches all 6 neighbors, regardless of any board
	/// boundaries.
	#[inline]
	pub fn coord_neighbors(self) -> [AxialCoords; 6] {
		[
			self + AxialCoords::new(-1, 0),
			self + AxialCoords::new(0, 1),
			self + AxialCoords::new(1, 1),
			self + AxialCoords::new(1, 0),
			self + AxialCoords::new(0, -1),
			self + AxialCoords::new(-1, -1)
		]
	}
}

impl CubeCoords {
	/// Creates new cube coordinates if they are valid.
	#[inline]
	pub fn new(x: i32, y: i32, z: i32) -> Option<Self> {
		if (x + y + z) == 0 {
			Some(CubeCoords { x: x, y: y, z: z })
		} else {
			None
		}
	}
	
	/// Fetches the x-coordinate
	#[inline]
	pub fn x(self) -> i32 { self.x }
	
	/// Fetches the y-coordinate
	#[inline]
	pub fn y(self) -> i32 { self.y }
	
	/// Fetches the z-coordinate
	#[inline]
	pub fn z(self) -> i32 { self.z }
}

impl Board {
	/// Fetches a reference to the field at the given
	/// coordinates. The coordinates can be of and type
	/// (e.g. axial/cube) as long as they are convertible
	/// to axial coordinates.
	pub fn field(&self, coords: impl Into<AxialCoords>) -> Option<&Field> {
		self.fields.get(&coords.into())
	}
	
	/// Tests whether a given position is obstructed.
	pub fn is_obstructed(&self, coords: impl Into<AxialCoords>) -> bool {
		self.field(coords).map(|f| f.is_obstructed).unwrap_or(true)
	}
	
	/// Fetches all fields owned by the given color.
	pub fn fields_owned_by(&self, color: PlayerColor) -> impl Iterator<Item=(&AxialCoords, &Field)> {
		self.fields.iter().filter(move |(_, f)| f.is_owned_by(color))
	}
	
	/// Fetches all fields.
	pub fn fields(&self) -> impl Iterator<Item=(&AxialCoords, &Field)> {
		self.fields.iter()
	}
	
	/// Tests whether the board contains the given coordinate.
	pub fn contains_coords(&self, coords: impl Into<AxialCoords>) -> bool {
		self.fields.contains_key(&coords.into())
	}
	
	/// Fetches the (existing) neighbor fields on the board.
	pub fn neighbors<'a>(&'a self, coords: impl Into<AxialCoords>) -> impl Iterator<Item=(AxialCoords, &Field)> + 'a {
		coords.into().coord_neighbors().iter().filter_map(|&c| self.field(c).map(|f| (c, f)))
	}
	
	/// Tests whether the bee of the given color has been placed.
	pub fn has_placed_bee(&self, color: PlayerColor) -> bool {
		self.fields().flat_map(|(_, f)| f.piece_stack()).any(|p| p.owner == color)
	}
	
	/// Performs an "inverted" depth-first search on the board
	/// starting at the given coordinates and removing visited
	/// locations from the set.
	fn inv_dfs_swarm(&self, coords: AxialCoords, unvisited: &mut HashSet<AxialCoords>) {
		if let Some(field) = self.field(coords).filter(|f| f.has_pieces()) {
			unvisited.remove(&coords);
			for (neighbor, _) in self.neighbors(coords) {
				if unvisited.contains(&neighbor) {
					self.inv_dfs_swarm(neighbor, unvisited)
				}
			}
		}
	}
	
	/// Performs a depth-first search on the board at the given
	/// position to test whether the swarm is connected.
	pub fn is_swarm_connected(&self) -> bool {
		let mut unvisited = self.fields.iter()
			.filter_map(|(c, f)| if f.has_pieces() { Some(c) } else { None })
			.cloned()
			.collect::<HashSet<AxialCoords>>();

		if let Some(start) = unvisited.iter().next() {
			self.inv_dfs_swarm(*start, &mut unvisited);
			unvisited.is_empty()
		} else {
			true // An empty swarm is connected
		}
	}
}

impl Field {
	/// Fetches the player color "owning" the field.
	pub fn owner(&self) -> Option<PlayerColor> { self.piece().map(|p| p.owner) }
	
	/// Tests whether the field is owned by the given owner.
	pub fn is_owned_by(&self, color: PlayerColor) -> bool { self.owner() == Some(color) }
	
	/// Fetches the top-most piece.
	pub fn piece(&self) -> Option<Piece> { self.piece_stack.last().cloned() }
	
	/// Tests whether the field contains pieces.
	pub fn has_pieces(&self) -> bool { !self.piece_stack.is_empty() }
	
	/// Fetches the piece stack.
	pub fn piece_stack(&self) -> &Vec<Piece> { &self.piece_stack }
	
	/// Pushes a piece onto the piece stack.
	pub fn push(&mut self, piece: Piece) { self.piece_stack.push(piece) }
	
	/// Pops a piece from the piece stack or
	/// returns `None` if the stack is empty.
	pub fn pop(&mut self) -> Option<Piece> { self.piece_stack.pop() }
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

	// Source: Partially translated from https://github.com/CAU-Kiel-Tech-Inf/socha/blob/8399e73673971427624a73ef42a1b023c69268ec/plugin/src/shared/sc/plugin2020/util/GameRuleLogic.kt

	fn validate_set_move(&self, color: PlayerColor, piece: Piece, destination_coords: impl Into<AxialCoords>) -> SCResult<()> {
		let destination = destination_coords.into();
		if !self.board.contains_coords(destination) {
			Err(format!("Move destination out of bounds: {:?}", destination).into())
		} else if self.board.field(destination).map(|f| f.is_obstructed).unwrap_or(true) {
			Err(format!("Move destination is obstructed: {:?}", destination).into())
		} else if !self.board.fields().any(|(_, f)| f.has_pieces()) {
			Ok(())
		} else if self.board.fields_owned_by(color).count() == 0 {
			let placed_next_to_opponent = self.board.fields_owned_by(color.opponent())
				.flat_map(|(&c, _)| self.board.neighbors(c))
				.any(|(c, _)| destination == c);
			if placed_next_to_opponent {
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
		unimplemented!()
	}
	
	//// Tests whether the given move is valid.
	pub fn validate_move(&self, color: PlayerColor, game_move: Move) -> SCResult<()> {
		match game_move {
			Move::SetMove { piece, destination } => self.validate_set_move(piece, destination),
			Move::DragMove { start, destination } => self.validate_drag_move(start, destination)
		}
	}
	
	/// Fetches a list of possible `SetMove`s.
	fn possible_set_moves(&self, color: PlayerColor) -> impl Iterator<Item=Move> {
		unimplemented!()
	}
	
	/// Fetches a list of possible `DragMove`s.
	fn possible_drag_moves(&self, color: PlayerColor) -> impl Iterator<Item=Move> {
		unimplemented!()
	}
	
	/// Fetches a list of possible moves for a given color.
	pub fn possible_moves(&self, color: PlayerColor) -> Vec<Move> {
		let moves = Vec::new();
		moves.extend(self.possible_set_moves(color));
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

// Operator overloads

impl Add for AxialCoords {
	type Output = Self;

	fn add(self, rhs: Self) -> Self { Self { x: self.x + rhs.x, y: self.y + rhs.y } }
}

impl Sub for AxialCoords {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self { Self { x: self.x - rhs.x, y: self.y - rhs.y } }
}

impl<R> Mul<R> for AxialCoords where R: Into<i32> {
	type Output = Self;
	
	fn mul(self, rhs: R) -> Self { Self { x: self.x * rhs.into(), y: self.y * rhs.into() } }
}

// General conversions

impl From<CubeCoords> for AxialCoords {
	fn from(coords: CubeCoords) -> Self { Self { x: coords.x, y: coords.y } }
}

impl From<AxialCoords> for CubeCoords {
	fn from(coords: AxialCoords) -> Self { Self { x: coords.x, y: coords.y, z: -(coords.x + coords.y) } }
}

impl FromStr for PlayerColor {
	type Err = SCError;

	fn from_str(raw: &str) -> SCResult<Self> {
		match raw {
			"RED" => Ok(Self::Red),
			"BLUE" => Ok(Self::Blue),
			_ => Err(format!("Did not recognize player color {}", raw).into())
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
		match raw {
			"ANT" => Ok(Self::Ant),
			"BEE" => Ok(Self::Bee),
			"BEETLE" => Ok(Self::Beetle),
			"GRASSHOPPER" => Ok(Self::Grasshopper),
			"SPIDER" => Ok(Self::Spider),
			_ => Err(format!("Did not recognize piece type {}", raw).into())
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
		Ok(Self {
			fields: node.child_by_name("fields")?
				.childs_by_name("field")
				.map(|f| Ok((
					CubeCoords {
						x: f.attribute("x")?.parse()?,
						y: f.attribute("y")?.parse()?,
						z: f.attribute("z")?.parse()?
					}.into(),
					Field::from_node(f)?
				)))
				.collect::<SCResult<HashMap<AxialCoords, Field>>>()?
		})
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
			Move::SetMove { piece, destination } => Self::new(
				"setmove",
				"",
				HashMap::new(),
				vec![piece.into(), CubeCoords::from(destination).into()]
			),
			Move::DragMove { start, destination } => Self::new(
				"dragmove",
				"",
				HashMap::new(),
				vec![CubeCoords::from(start).into(), CubeCoords::from(destination).into()]
			)
		}
	}
}

impl From<Piece> for XmlNode {
	fn from(piece: Piece) -> Self {
		Self::new(
			"piece",
			"",
			hashmap!["owner".to_owned() => piece.owner.into(), "type".to_owned() => piece.piece_type.into()],
			vec![]
		)
	}
}

impl From<CubeCoords> for XmlNode {
	fn from(coords: CubeCoords) -> Self {
		Self::new(
			"CubeCoordinates",
			"",
			hashmap!["x".to_owned() => coords.x.to_string(), "y".to_owned() => coords.y.to_string(), "z".to_owned() => coords.z.to_string()],
			vec![]
		)
	}
}
