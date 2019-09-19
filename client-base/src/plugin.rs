use std::str::FromStr;
use std::fmt::Debug;
use crate::xml_node::{FromXmlNode, XmlNode};
use crate::util::HasOpponent;
use crate::error::SCError;

/// An "type family" trait that defines types
/// which represent various parts of a game.
pub trait SCPlugin: Debug {
	type PlayerColor: Copy + Debug + Eq + HasOpponent + FromStr<Err=SCError>;
	type Player<'a>: Clone + Debug + Eq + FromXmlNode;
	type GameState<'a>: Clone + Debug + Eq + FromXmlNode;
	type Move: Clone + Debug + Eq + Into<XmlNode>;
}
