//! The data structures used by the XML protocol.

use std::str::FromStr;
use crate::xml_node::{FromXmlNode, XmlNode};
use crate::plugin::SCPlugin;
use crate::util::SCResult;
use crate::hashmap;

/// A message indicating that the client
/// has joined a room with the specified id.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Joined {
	pub room_id: String
}

/// A message indicating that the client
/// has left a room with the specified id.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Left {
	pub room_id: String
}

/// A message in a room together with some data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Room<P> where P: SCPlugin {
	pub room_id: String,
	pub data: Data<P>
}

/// A polymorphic container for game data
/// used by the protocol. It is parameterized
/// by the player color (`C`), the game state (`S`)
/// and the player structure (`P`). These types
/// are implemented independently of the base
/// protocol for each year's game.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Data<P> where P: SCPlugin {
	WelcomeMessage { color: P::PlayerColor },
	Memento { state: P::GameState },
	Move(P::Move),
	MoveRequest,
	GameResult(GameResult<P>)
}

/// The final result of a game.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameResult<P> where P: SCPlugin {
	pub definition: ScoreDefinition,
	pub scores: Vec<PlayerScore>,
	pub winners: Vec<P::Player>
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

impl FromXmlNode for Joined {
	fn from_node(node: &XmlNode) -> SCResult<Self> { Ok(Self { room_id: node.attribute("roomId")?.to_owned() }) }
}

impl FromXmlNode for Left {
	fn from_node(node: &XmlNode) -> SCResult<Self> { Ok(Self { room_id: node.attribute("roomId")?.to_owned() }) }
}

impl<P> FromXmlNode for Room<P> where P: SCPlugin {
	fn from_node(node: &XmlNode) -> SCResult<Self> {
		Ok(Self {
			room_id: node.attribute("roomId")?.to_owned(),
			data: <Data<P>>::from_node(node.child_by_name("data")?)?
		})
	}
}

impl<P> FromXmlNode for Data<P> where P: SCPlugin {
	fn from_node(node: &XmlNode) -> SCResult<Self> {
		let class = node.attribute("class")?;
		match class {
			"welcomeMessage" => Ok(Self::WelcomeMessage { color: node.attribute("color")?.parse()? }),
			"memento" => Ok(Self::Memento { state: P::GameState::from_node(node.child_by_name("state")?)? }),
			"sc.framework.plugins.protocol.MoveRequest" => Ok(Self::MoveRequest),
			_ => Err(format!("Unrecognized data class: {}", class).into())
		}
	}
}

impl<P> FromXmlNode for GameResult<P> where P: SCPlugin, P::Player: FromXmlNode {
	fn from_node(node: &XmlNode) -> SCResult<Self> {
		Ok(Self {
			definition: ScoreDefinition::from_node(node.child_by_name("definition")?)?,
			scores: node.childs_by_name("score").map(PlayerScore::from_node).collect::<SCResult<_>>()?,
			winners: node.childs_by_name("winner").map(P::Player::from_node).collect::<SCResult<_>>()?
		})
	}
}

impl FromXmlNode for ScoreDefinition {
	fn from_node(node: &XmlNode) -> SCResult<Self> {
		Ok(Self {
			fragments: node.childs_by_name("fragment").map(ScoreFragment::from_node).collect::<SCResult<_>>()?
		})
	}
}

impl FromXmlNode for ScoreFragment {
	fn from_node(node: &XmlNode) -> SCResult<Self> {
		Ok(Self {
			name: node.attribute("name")?.to_owned(),
			aggregation: node.attribute("aggregation")?.parse()?,
			relevant_for_ranking: node.child_by_name("relevantForRanking")?.data().parse()?
		})
	}
}

impl FromXmlNode for PlayerScore {
	fn from_node(node: &XmlNode) -> SCResult<Self> {
		Ok(Self {
			cause: node.attribute("cause")?.parse()?,
			reason: node.attribute("reason")?.to_owned()
		})
	}
}

impl<P> From<Room<P>> for SCResult<XmlNode> where P: SCPlugin {
	fn from(room: Room<P>) -> SCResult<XmlNode> {
		Ok(XmlNode::new(
			"room",
			"",
			hashmap!["roomId".to_owned() => room.room_id],
			vec![<SCResult<XmlNode>>::from(room.data)?]
		))
	}
}

impl<P> From<Data<P>> for SCResult<XmlNode> where P: SCPlugin {
	fn from(data: Data<P>) -> SCResult<XmlNode> {
		let name = "data";
		match data {
			Data::Move(game_move) => Ok(XmlNode::new(
				name,
				"",
				hashmap!["class".to_owned() => "move".to_owned()],
				vec![game_move.into()]
			)),
			_ => Err(format!("{:?} can currently not be serialized", data).into())
		}
	}
}
