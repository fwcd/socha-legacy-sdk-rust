use crate::{plugin::SCPlugin, util::SCResult, xml_node::FromXmlNode, xml_node::XmlNode};

use super::{PlayerScore, ScoreDefinition};

/// The final result of a game.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameResult<P> where P: SCPlugin {
    pub definition: ScoreDefinition,
    pub scores: Vec<PlayerScore>,
    pub winners: Vec<P::Player>
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
