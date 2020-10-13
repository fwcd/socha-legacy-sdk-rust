use socha_client_base::{util::SCResult, xml_node::{FromXmlNode, XmlNode}};

use super::{Color, Coordinates, Field, Piece};

pub const BOARD_SIZE: usize = 20;
const BOARD_CORNERS: [Coordinates; 4] = [
    Coordinates { x: 0, y: 0 },
    Coordinates { x: 0, y: BOARD_SIZE as i32 - 1 },
    Coordinates { x: BOARD_SIZE as i32 - 1, y: 0 },
    Coordinates { x: BOARD_SIZE as i32 - 1, y: BOARD_SIZE as i32 - 1 }
];

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

    // Note that the following bound checking methods
    // do not depend on the self-instance, but are declared
    // as instance methods anyways. This is intentional to
    // have code depend as little as possible on the constness
    // of the the board size.

    /// Checks whether the given coordinates are in the board's bounds.
    pub fn is_in_bounds(&self, coordinates: Coordinates) -> bool {
           coordinates.x >= 0
        && coordinates.y >= 0
        && coordinates.x < BOARD_SIZE as i32
        && coordinates.y < BOARD_SIZE as i32
    }

    /// Fetches the board's corners.
    pub fn corners(&self) -> impl Iterator<Item=Coordinates> {
        BOARD_CORNERS.iter().cloned()
    }

    /// Checks whether a coordinate is on a corner.
    pub fn is_on_corner(&self, position: Coordinates) -> bool {
        BOARD_CORNERS.contains(&position)
    }

    /// Fetches the color at the given position.
    pub fn get(&self, position: Coordinates) -> Color {
        // TODO: This is very inefficient and would be much better handled using a matrix
        self.fields.iter().find(|f| f.position == position).map(|f| f.content).unwrap_or_default()
    }

    /// Places the color at the given position.
    pub fn set(&mut self, position: Coordinates, color: Color) {
        // TODO: This is very inefficient and would be much better handled using a matrix
        match self.fields.iter_mut().find(|f| f.position == position) {
            Some(field) => field.content = color,
            None => self.fields.push(Field { position, content: color })
        }
    }

    /// Places the given piece on the board WITH NO ADDITIONAL CHECKS.
    pub fn place(&mut self, piece: &Piece) {
        for position in piece.coordinates() {
            self.set(position, piece.color);
        }
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
