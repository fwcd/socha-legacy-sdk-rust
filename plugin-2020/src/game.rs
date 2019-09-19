//! The game structures for the "Hive" game.

use std::collections::HashMap;
use std::str::FromStr;
use socha_client_base::util::{SCResult, HasOpponent};
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AxialCoords {
	pub x: i32,
	pub y: i32
}

/// Cube coordinates on the hex grid.
/// These are used by the protocol internally.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CubeCoords {
	pub x: i32,
	pub y: i32,
	pub z: i32
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
	pub is_obstructed: bool
}

/// A transition between two game states.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Move<C=AxialCoords> {
	SetMove { piece: Piece, destination: C },
	DragMove { start: C, end: C }
}

/// A game piece.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Piece {
	pub owner: PlayerColor,
	pub piece_type: PieceType
}

/// A game piece type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PieceType {
	Ant,
	Bee,
	Beetle,
	Grasshopper,
	Spider
}

// General implementations

impl Board {
	/// Fetches a reference to the field at the given
	/// coordinates. The coordinates can be of and type
	/// (e.g. axial/cube) as long as they are convertible
	/// to axial coordinates.
	pub fn field(&self, coords: impl Into<AxialCoords>) -> Option<&Field> {
		self.fields.get(&coords.into())
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
