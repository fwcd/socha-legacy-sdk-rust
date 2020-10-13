use socha_client_base::{util::SCResult, xml_node::{FromXmlNode, XmlNode}};

use super::Field;

/// The game board is a 20x20 grid of fields with colors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    // TODO: More efficient representation, e.g. using a 2D matrix of colors
    fields: Vec<Field>
}

impl FromXmlNode for Board {
    fn from_node(node: &XmlNode) -> SCResult<Self> {
        Ok(Self {
            fields: node.childs_by_name("field").map(Field::from_node).collect::<Result<_, _>>()?
        })
    }
}
