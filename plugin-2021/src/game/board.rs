use socha_client_base::{util::SCResult, xml_node::{FromXmlNode, XmlNode}};

use super::{Color, Coordinates, Field};

pub const BOARD_SIZE: usize = 20;

/// The game board is a 20x20 grid of fields with colors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    // TODO: More efficient representation, e.g. using a 2D matrix of colors
    fields: Vec<Field>
}

impl Board {
    /// Creates an empty board.
    pub fn new() -> Self {
        Self { fields: Vec::new() }
    }

    /// Checks whether the given coordinates are in the board's bounds.
    pub fn is_in_bounds(&self, coordinates: Coordinates) -> bool {
           coordinates.x >= 0
        && coordinates.y >= 0
        && coordinates.x < BOARD_SIZE as i32
        && coordinates.y < BOARD_SIZE as i32
    }

    /// Fetches the color at the given position.
    pub fn get(&self, position: Coordinates) -> Color {
        self.fields.iter().find(|f| f.position == position).map(|f| f.content).unwrap_or_default()
    }

    /// Checks whether the given position is obstructed.
    pub fn is_obstructed(&self, position: Coordinates) -> bool {
        self.fields.iter().any(|f| f.position == position && f.content != Color::None)
    }

    /// Checks whether the position touches another border of same color.
    pub fn borders_on_color(&self, position: Coordinates, color: Color) -> bool {
        [
            Coordinates::new(1, 0),
            Coordinates::new(0, 1),
            Coordinates::new(-1, 0),
            Coordinates::new(0, -1)
        ].iter().any(|&o| self.get(position + o) == color)
    }

    /// Checks whether the position touches another corner of same color.
    pub fn corners_on_color(&self, position: Coordinates, color: Color) -> bool {
        [
            Coordinates::new(1, 1),
            Coordinates::new(1, 1),
            Coordinates::new(-1, 1),
            Coordinates::new(1, -1)
        ].iter().any(|&o| self.get(position + o) == color)
    }
}

impl FromXmlNode for Board {
    fn from_node(node: &XmlNode) -> SCResult<Self> {
        Ok(Self {
            fields: node.childs_by_name("field").map(Field::from_node).collect::<Result<_, _>>()?
        })
    }
}
