use socha_client_base::{util::SCResult, xml_node::{FromXmlNode, XmlNode}};

use super::Color;

/// A field on the board holding a color.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    pub x: u32,
    pub y: u32,
    pub content: Color
}

impl FromXmlNode for Field {
    fn from_node(node: &XmlNode) -> SCResult<Self> {
        Ok(Self {
            x: node.attribute("x")?.parse()?,
            y: node.attribute("y")?.parse()?,
            content: node.attribute("content")?.parse()?
        })
    }
}
