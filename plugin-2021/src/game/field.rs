use socha_client_base::{util::SCResult, xml_node::{FromXmlNode, XmlNode}};

use super::{Color, Coordinates};

/// A field on the board holding a color.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    pub position: Coordinates,
    pub content: Color
}

impl FromXmlNode for Field {
    fn from_node(node: &XmlNode) -> SCResult<Self> {
        Ok(Self {
            position: Coordinates::new(
                node.attribute("x")?.parse()?,
                node.attribute("y")?.parse()?
            ),
            content: node.attribute("content")?.parse()?
        })
    }
}
