use std::str::FromStr;
use std::fmt::Debug;
use crate::xml_node::{FromXmlNode, XmlNode};
use crate::util::HasOpponent;
use crate::error::SCError;

/// An "type family" trait that defines types
/// which represent various parts of a game.
pub trait SCPlugin: Debug {
	type PlayerColor: Copy + Debug + Eq + HasOpponent + FromStr<Err=SCError>;
	type Player: Clone + Debug + Eq + FromXmlNode;
	type GameState: Clone + Debug + Eq + FromXmlNode + HasPlayerColor<PlayerColor=Self::PlayerColor> + HasTurn;
	type Move: Clone + Debug + Eq + Into<XmlNode>;
	
	/// Fetches the 'gameType' used during
	/// the protocol handshake.
	fn protocol_game_type<'a>() -> &'a str;
}

/// Indicates that the value has an "associated" player color.
/// The plugin-specific `GameState` should return the current player
/// color when implementing this trait.
pub trait HasPlayerColor {
	type PlayerColor;

	/// Fetches the associated player color.
	fn player_color(&self) -> Self::PlayerColor;
}

/// Indicates that the value has a turn.
pub trait HasTurn {
	/// Fetches the turn.
	fn turn(&self) -> u32;
}
