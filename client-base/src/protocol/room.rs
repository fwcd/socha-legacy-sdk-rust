use std::convert::TryFrom;

use crate::{error::SCError, plugin::SCPlugin, util::SCResult, xml_node::FromXmlNode, xml_node::XmlNode};

use super::Data;

/// A message in a room together with some data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Room<P> where P: SCPlugin {
    pub room_id: String,
    pub data: Data<P>
}

impl<P> FromXmlNode for Room<P> where P: SCPlugin {
    fn from_node(node: &XmlNode) -> SCResult<Self> {
        Ok(Self {
            room_id: node.attribute("roomId")?.to_owned(),
            data: <Data<P>>::from_node(node.child_by_name("data")?)?
        })
    }
}

impl<P> TryFrom<Room<P>> for XmlNode where P: SCPlugin {
    type Error = SCError;

    fn try_from(room: Room<P>) -> SCResult<XmlNode> {
        Ok(XmlNode::new("room")
            .attribute("roomId", room.room_id)
            .try_child(room.data)?
            .into())
    }
}
