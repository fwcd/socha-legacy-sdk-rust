use std::{collections::HashMap, fmt, str::FromStr};

use lazy_static::lazy_static;
use socha_client_base::{error::SCError, util::SCResult, xml_node::FromXmlNode, xml_node::XmlNode};

use super::{BOARD_SIZE, Coordinates, Rotation};

lazy_static! {
    pub static ref PIECE_SHAPES: [PieceShape; 21] = [
        PieceShape::new("MONO", vec![Coordinates::new(0, 0)]),
        PieceShape::new("DOMINO", vec![Coordinates::new(0, 0), Coordinates::new(1, 0)]),
        PieceShape::new("TRIO_L", vec![Coordinates::new(0, 0), Coordinates::new(0, 1), Coordinates::new(1, 1)]),
        PieceShape::new("TRIO_I", vec![Coordinates::new(0, 0), Coordinates::new(0, 1), Coordinates::new(0, 2)]),
        PieceShape::new("TETRO_O", vec![Coordinates::new(0, 0), Coordinates::new(1, 0), Coordinates::new(0, 1), Coordinates::new(1, 1)]),
        PieceShape::new("TETRO_T", vec![Coordinates::new(0, 0), Coordinates::new(1, 0), Coordinates::new(2, 0), Coordinates::new(1, 1)]),
        PieceShape::new("TETRO_I", vec![Coordinates::new(0, 0), Coordinates::new(0, 1), Coordinates::new(0, 2), Coordinates::new(0, 3)]),
        PieceShape::new("TETRO_L", vec![Coordinates::new(0, 0), Coordinates::new(0, 1), Coordinates::new(0, 2), Coordinates::new(1, 2)]),
        PieceShape::new("TETRO_Z", vec![Coordinates::new(0, 0), Coordinates::new(1, 0), Coordinates::new(1, 1), Coordinates::new(2, 1)]),
        PieceShape::new("PENTO_L", vec![Coordinates::new(0, 0), Coordinates::new(0, 1), Coordinates::new(0, 2), Coordinates::new(0, 3), Coordinates::new(1, 3)]),
        PieceShape::new("PENTO_T", vec![Coordinates::new(0, 0), Coordinates::new(1, 0), Coordinates::new(2, 0), Coordinates::new(1, 1), Coordinates::new(1, 2)]),
        PieceShape::new("PENTO_V", vec![Coordinates::new(0, 0), Coordinates::new(0, 1), Coordinates::new(0, 2), Coordinates::new(1, 2), Coordinates::new(2, 2)]),
        PieceShape::new("PENTO_S", vec![Coordinates::new(1, 0), Coordinates::new(2, 0), Coordinates::new(3, 0), Coordinates::new(0, 1), Coordinates::new(1, 1)]),
        PieceShape::new("PENTO_Z", vec![Coordinates::new(0, 0), Coordinates::new(1, 0), Coordinates::new(1, 1), Coordinates::new(1, 2), Coordinates::new(2, 2)]),
        PieceShape::new("PENTO_I", vec![Coordinates::new(0, 0), Coordinates::new(0, 1), Coordinates::new(0, 2), Coordinates::new(0, 3), Coordinates::new(0, 4)]),
        PieceShape::new("PENTO_P", vec![Coordinates::new(0, 0), Coordinates::new(1, 0), Coordinates::new(0, 1), Coordinates::new(1, 1), Coordinates::new(0, 2)]),
        PieceShape::new("PENTO_W", vec![Coordinates::new(0, 0), Coordinates::new(0, 1), Coordinates::new(1, 1), Coordinates::new(1, 2), Coordinates::new(2, 2)]),
        PieceShape::new("PENTO_U", vec![Coordinates::new(0, 0), Coordinates::new(0, 1), Coordinates::new(1, 1), Coordinates::new(2, 1), Coordinates::new(2, 0)]),
        PieceShape::new("PENTO_R", vec![Coordinates::new(0, 1), Coordinates::new(1, 1), Coordinates::new(1, 2), Coordinates::new(2, 1), Coordinates::new(2, 0)]),
        PieceShape::new("PENTO_X", vec![Coordinates::new(1, 0), Coordinates::new(0, 1), Coordinates::new(1, 1), Coordinates::new(2, 1), Coordinates::new(1, 2)]),
        PieceShape::new("PENTO_Y", vec![Coordinates::new(0, 1), Coordinates::new(1, 0), Coordinates::new(1, 1), Coordinates::new(1, 2), Coordinates::new(1, 3)])
    ];

    pub static ref PIECE_SHAPES_BY_NAME: HashMap<String, PieceShape> = {
        let mut m = HashMap::new();
        for piece in PIECE_SHAPES.iter() {
            m.insert(piece.name.to_owned(), piece.clone());
        }
        m
    };
}

const MAX_SIDE_LENGTH: i32 = 5;

/// An efficient representation of a piece shape's normalized coordinates.
/// Since every piece shape is less than 5x5 is size, we can represent it
/// using a 5x5 bit-matrix:
///
///     +---+---+---+---+----+
///     | 0 | 1 | 2 | 3 |  4 |
///     +---+---+---+---+----+
///     | 5 | 6 |            |
///     +---+---+    ...     |
///     |                    |
///     +               +----+
///     |               | 24 |
///     +---+---+---+---+----+
///
/// These bits are stored in the right-end of of a 32-bit integer.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct CoordinateSet {
    bits: u32
}

impl CoordinateSet {
    pub fn new() -> Self {
        Self { bits: 0 }
    }

    fn index_of(coordinates: Coordinates) -> usize {
        assert!(coordinates.x >= 0 && coordinates.y >= 0, "Coordinates have to be positive!");
        assert!(coordinates.y < MAX_SIDE_LENGTH && coordinates.y < MAX_SIDE_LENGTH, "Coordinates are out of bounds!");

        let i = (coordinates.y * MAX_SIDE_LENGTH) + coordinates.x;
        i as usize
    }

    /// Inserts a pair of coordinates (inside the 5x5 box) into the set.
    pub fn insert(&mut self, coordinates: Coordinates) {
        self.bits |= 1 << Self::index_of(coordinates);
    }

    /// Checks whether the set contains a given pair of coordinates.
    pub fn contains(&self, coordinates: Coordinates) -> bool {
           coordinates.x >= 0
        && coordinates.y >= 0
        && coordinates.x < MAX_SIDE_LENGTH
        && coordinates.y < MAX_SIDE_LENGTH
        && ((self.bits >> Self::index_of(coordinates)) & 1) == 1
    }
}

impl<I> From<I> for CoordinateSet where I: Iterator<Item=Coordinates> {
    fn from(coordinates: I) -> Self {
        let mut set = Self::new();

        for coordinates in coordinates {
            set.insert(coordinates);
        }

        set
    }
}

struct CoordinateSetIterator {
    bits: u32,
    i: i32
}

impl Iterator for CoordinateSetIterator {
    type Item = Coordinates;

    fn next(&mut self) -> Option<Self::Item> {
        while self.i < (MAX_SIDE_LENGTH * MAX_SIDE_LENGTH) {
            let i = self.i;
            let bits = self.bits;

            self.bits >>= 1;
            self.i += 1;

            if (bits & 1) == 1 {
                return Some(Coordinates::new(i % MAX_SIDE_LENGTH, i / MAX_SIDE_LENGTH));
            }
        }
        
        None
    }
}

impl IntoIterator for CoordinateSet {
    type Item = Coordinates;
    type IntoIter = CoordinateSetIterator;

    fn into_iter(self) -> Self::IntoIter {
        CoordinateSetIterator { bits: self.bits, i: 0 }
    }
}

/// Represents a shape in Blokus. There are 21 different kinds of these.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PieceShape {
    /// The shape's internal name.
    name: String,
    /// The normalized coordinates that make up the shape.
    coordinates: CoordinateSet
}

impl PieceShape {
    fn new(name: &str, coordinates: impl IntoIterator<Item=Coordinates>) -> Self {
        Self { name: name.to_owned(), coordinates: CoordinateSet::from(coordinates.into_iter()) }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// A list of occupied fields, with the upper left corner being
    /// the origin (0, 0), the x-axis pointed right and the y-axis pointed downwards
    pub fn coordinates(&self) -> impl Iterator<Item=Coordinates> {
        self.coordinates.into_iter()
    }

    /// Mirrors this shape by negating all coordinates.
    fn mirror(&self) -> Self {
        Self::new(self.name(), self.coordinates().map(|c| -c))
    }

    /// Turns this piece 90 degrees to the right.
    fn turn_right(&self) -> Self {
        Self::new(self.name(), self.coordinates().map(|c| c.turn_right()))
    }

    /// Turns this piece 90 degrees to the left.
    fn turn_left(&self) -> Self {
        Self::new(self.name(), self.coordinates().map(|c| c.turn_left()))
    }

    /// Adjusts the coordinates of this piece shape to be relative
    /// to its minimum coords.
    fn align(&self) -> Self {
        let min_coords = self.coordinates().fold(Coordinates::new(BOARD_SIZE as i32, BOARD_SIZE as i32), |m, c| m.min(c));
        Self::new(self.name(), self.coordinates().map(|c| c - min_coords))
    }

    /// Performs a rotation of this piece shape.
    pub fn rotate(&self, rotation: Rotation) -> Self {
        match rotation {
            Rotation::None => self.clone(),
            Rotation::Mirror => self.mirror().align(),
            Rotation::Right => self.turn_right().align(),
            Rotation::Left => self.turn_left().align()
        }
    }
}

impl FromStr for PieceShape {
    type Err = SCError;

    fn from_str(raw: &str) -> SCResult<Self> {
        Ok(PIECE_SHAPES_BY_NAME.get(raw).ok_or_else(|| format!("Could not parse shape {}", raw))?.clone())
    }
}

impl fmt::Display for PieceShape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl FromXmlNode for PieceShape {
    fn from_node(node: &XmlNode) -> SCResult<Self> {
        node.content().parse()
    }
}