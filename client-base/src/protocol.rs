//! The data structures used by the XML protocol.

use std::str::FromStr;
use crate::xml_node::{FromXmlNode, XmlNode};
use crate::util::SCResult;
use crate::error::SCError;

/// A message indicating that the client
/// has joined a room with the specified id.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Joined {
	pub room_id: String
}

/// A message in a room together with some data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Room<C, S, P> {
	pub room_id: String,
	pub data: Data<C, S, P>
}

/// A polymorphic container for game data
/// used by the protocol. It is parameterized
/// by the player color (`C`), the game state (`S`)
/// and the player structure (`P`). These types
/// are implemented independently of the base
/// protocol for each year's game.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Data<C, S, P> {
	WelcomeMessage { color: C },
	Memento { state: S },
	MoveRequest,
	GameResult { result: GameResult<P> }
}

/// The final result of a game.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameResult<P> {
	pub definition: ScoreDefinition,
	pub scores: Vec<PlayerScore>,
	pub winners: Vec<P>
}

/// The definition of a score.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScoreDefinition {
	pub fragments: Vec<ScoreFragment>
}

/// A single score fragment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScoreFragment {
	pub name: String,
	pub aggregation: ScoreAggregation,
	pub relevant_for_ranking: bool
}

/// Determines how scores should be aggregated (e.g. summed up or averaged over).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScoreAggregation {
	Sum,
	Average
}

/// Determines the cause of a game score.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScoreCause {
	Regular,
	Left,
	RuleViolation,
	SoftTimeout,
	HardTimeout,
	Unknown
}

/// The score of a game player.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerScore {
	pub cause: ScoreCause,
	pub reason: String
}

// General conversions

impl FromStr for ScoreAggregation {
	type Err = String;

	fn from_str(raw: &str) -> Result<Self, String> {
		match raw {
			"SUM" => Ok(Self::Sum),
			"AVERAGE" => Ok(Self::Average),
			_ => Err(format!("Unknown score aggregation: {}", raw))
		}
	}
}

impl FromStr for ScoreCause {
	type Err = String;

	fn from_str(raw: &str) -> Result<Self, String> {
		match raw {
			"REGULAR" => Ok(Self::Regular),
			"LEFT" => Ok(Self::Left),
			"RULE_VIOLATION" => Ok(Self::RuleViolation),
			"SOFT_TIMEOUT" => Ok(Self::SoftTimeout),
			"HARD_TIMEOUT" => Ok(Self::HardTimeout),
			"UNKNOWN" => Ok(Self::Unknown),
			_ => Err(format!("Unknown score cause: {}", raw))
		}
	}
}

// XML conversions

impl<'a> FromXmlNode<'a> for Joined {
	fn from_node(node: &'a XmlNode) -> SCResult<Self> { Ok(Self { room_id: node.attribute("room_id")?.to_owned() }) }
}

impl<'a, C, S, P> FromXmlNode<'a> for Room<C, S, P> where Data<C, S, P>: FromXmlNode<'a> {
	fn from_node(node: &'a XmlNode) -> SCResult<Self> {
		Ok(Self {
			room_id: node.attribute("room_id")?.to_owned(),
			data: <Data<C, S, P>>::from_node(node.child_by_name("data")?)?
		})
	}
}

impl<'a, C, S, P> FromXmlNode<'a> for Data<C, S, P> where C: FromStr, SCError: From<C::Err>, S: FromXmlNode<'a> {
	fn from_node(node: &'a XmlNode) -> SCResult<Self> {
		let class = node.attribute("class")?;
		match class {
			"welcomeMessage" => Ok(Self::WelcomeMessage { color: node.attribute("color")?.parse()? }),
			"memento" => Ok(Self::Memento { state: S::from_node(node.child_by_name("state")?)? }),
			"sc.framework.plugins.protocol.MoveRequest" => Ok(Self::MoveRequest),
			_ => Err(format!("Unrecognized data class: {}", class).into())
		}
	}
}

impl<'a, P> FromXmlNode<'a> for GameResult<P> where P: FromXmlNode<'a> {
	fn from_node(node: &'a XmlNode) -> SCResult<Self> {
		Ok(Self {
			definition: ScoreDefinition::from_node(node.child_by_name("definition")?)?,
			scores: node.childs_by_name("score").map(PlayerScore::from_node).collect::<SCResult<_>>()?,
			winners: node.childs_by_name("winner").map(P::from_node).collect::<SCResult<_>>()?
		})
	}
}

impl<'a> FromXmlNode<'a> for ScoreDefinition {
	fn from_node(node: &'a XmlNode) -> SCResult<Self> {
		Ok(Self {
			fragments: node.childs_by_name("fragment").map(ScoreFragment::from_node).collect::<SCResult<_>>()?
		})
	}
}

impl<'a> FromXmlNode<'a> for ScoreFragment {
	fn from_node(node: &'a XmlNode) -> SCResult<Self> {
		Ok(Self {
			name: node.attribute("name")?.to_owned(),
			aggregation: node.attribute("aggregation")?.parse()?,
			relevant_for_ranking: node.child_by_name("relevantForRanking")?.data().parse()?
		})
	}
}


impl<'a> FromXmlNode<'a> for PlayerScore {
	fn from_node(node: &'a XmlNode) -> SCResult<Self> {
		Ok(Self {
			cause: node.attribute("cause")?.parse()?,
			reason: node.attribute("reason")?.to_owned()
		})
	}
}

// TODO: Into XML node conversions
