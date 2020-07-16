use socha_client_base::{error::SCError, util::{SCResult, HasOpponent}};
use std::{str::FromStr, collections::HashSet, convert::TryFrom, ops::{Add, Sub, Neg}};

// Structures

/// A color in the game.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Color {
    None,
    Blue,
    Yellow,
    Red,
    Green
}

/// A player's team.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Team {
    None,
    One,
    Two
}

/// Metadata about a player.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Player {
    pub team: Team,
    pub display_name: String
}

/// A snapshot of the game's state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    pub turn: u32,
    pub red_player: Player,
    pub blue_player: Player,
    pub board: Board,
    pub current_team: Team
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    pub fields: Vec<Vec<Color>>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Move {
    pub piece: Piece,
    /// The coordinates the upper left corner this piece is placed on.
    pub position: Coordinates
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Piece {
    pub kind: usize,
    pub rotation: Rotation,
    pub color: Color
}

/// Represents a shape in Blokus. There are 21 different kinds of these.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PieceShape {
    coordinates: HashSet<Coordinates>
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Rotation {
    None,
    Right,
    Mirror,
    Left
}

/// A point in 2D-space. The x-axis
/// usually points to the right while
/// the y-axis points downwards.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32
}

impl HasOpponent for Team {
    fn opponent(self) -> Self {
        match self {
            Self::None => Self::None,
            Self::Red => Self::Blue,
            Self::Blue => Self::Red
        }
    }
}

// Constants

pub const BOARD_SIZE: usize = 20;
pub const PIECE_SHAPES: [PieceShape] = [
    PieceShape::new(&[Coordinates::new(0, 0)]),
    PieceShape::new(&[Coordinates::new(0, 0), Coordinates::new(1, 0)]),
    PieceShape::new(&[Coordinates::new(0, 0), Coordinates::new(1, 0), Coordinates::new(1, 1)]),
    PieceShape::new(&[Coordinates::new(0, 0), Coordinates::new(1, 0), Coordinates::new(2, 0)]),
    PieceShape::new(&[Coordinates::new(0, 0), Coordinates::new(1, 0), Coordinates::new(0, 1), Coordinates::new(1, 1)]),
    PieceShape::new(&[Coordinates::new(1, 0), Coordinates::new(0, 1), Coordinates::new(1, 1), Coordinates::new(2, 1)]),
    PieceShape::new(&[Coordinates::new(0, 0), Coordinates::new(1, 0), Coordinates::new(2, 0), Coordinates::new(3, 0)]),
    PieceShape::new(&[Coordinates::new(2, 0), Coordinates::new(0, 1), Coordinates::new(1, 1), Coordinates::new(2, 1)]),
    PieceShape::new(&[Coordinates::new(1, 0), Coordinates::new(2, 0), Coordinates::new(0, 1), Coordinates::new(1, 1)]),
    PieceShape::new(&[Coordinates::new(0, 0), Coordinates::new(0, 1), Coordinates::new(1, 1), Coordinates::new(2, 1), Coordinates::new(3, 1)]),
    PieceShape::new(&[Coordinates::new(1, 0), Coordinates::new(1, 1), Coordinates::new(0, 2), Coordinates::new(1, 2), Coordinates::new(2, 2)]),
    PieceShape::new(&[Coordinates::new(0, 0), Coordinates::new(0, 1), Coordinates::new(0, 2), Coordinates::new(1, 2), Coordinates::new(2, 2)]),
    PieceShape::new(&[Coordinates::new(1, 0), Coordinates::new(2, 0), Coordinates::new(3, 0), Coordinates::new(0, 1), Coordinates::new(1, 1)]),
    PieceShape::new(&[Coordinates::new(2, 0), Coordinates::new(0, 1), Coordinates::new(1, 1), Coordinates::new(2, 1), Coordinates::new(0, 2)]),
    PieceShape::new(&[Coordinates::new(0, 0), Coordinates::new(0, 1), Coordinates::new(0, 2), Coordinates::new(0, 3), Coordinates::new(0, 4)]),
    PieceShape::new(&[Coordinates::new(0, 0), Coordinates::new(0, 1), Coordinates::new(1, 1), Coordinates::new(0, 2), Coordinates::new(1, 2)]),
    PieceShape::new(&[Coordinates::new(1, 0), Coordinates::new(2, 0), Coordinates::new(0, 1), Coordinates::new(1, 1), Coordinates::new(0, 2)]),
    PieceShape::new(&[Coordinates::new(0, 0), Coordinates::new(1, 0), Coordinates::new(0, 1), Coordinates::new(0, 2), Coordinates::new(1, 2)]),
    PieceShape::new(&[Coordinates::new(1, 0), Coordinates::new(2, 0), Coordinates::new(0, 1), Coordinates::new(1, 1), Coordinates::new(1, 2)]),
    PieceShape::new(&[Coordinates::new(1, 0), Coordinates::new(0, 1), Coordinates::new(1, 1), Coordinates::new(2, 1), Coordinates::new(1, 2)]),
    PieceShape::new(&[Coordinates::new(1, 0), Coordinates::new(0, 1), Coordinates::new(1, 1), Coordinates::new(2, 1), Coordinates::new(3, 1)])
];

// Implementations

impl Coordinates {
    /// Creates new coordinates.
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Rotates these coordinates 90 degrees clockwise.
    pub fn turn_right(self) -> Self {
        Self::new(-self.y, self.x)
    }

    /// Rotates these coordinates 90 degrees counter-clockwise.
    pub fn turn_left(self) -> Self {
        Self::new(self.y, -self.x)
    }

    /// Finds the minimum with another point.
    pub fn min(self, other: Coordinates) -> Self {
        Self::new(self.x.min(other.x), self.y.min(other.y))
    }

    /// Finds the maximum with another point.
    pub fn max(self, other: Coordinates) -> Self {
        Self::new(self.x.max(other.x), self.y.max(other.y))
    }
}

impl Piece {
    pub fn new(kind: usize, rotation: Rotation, color: Color) -> Self {
        Self { kind, rotation, color }
    }
}

impl PieceShape {
    fn new(coordinates: impl IntoIterator<Item=Coordinates>) -> Self {
        Self { coordinates: coordinates.into_iter().collect() }
    }

    /// A list of occupied fields, with the upper left corner being
    /// the origin (0, 0), the x-axis pointed right and the y-axis pointed downwards
    pub fn coordinates(&self) -> &HashSet<Coordinates> {
        self.coordinates
    }

    /// Mirrors this shape by negating all coordinates.
    fn mirror(&self) -> Self {
        Self::new(self.coordinates.iter().map(|c| -c))
    }

    /// Turns this piece 90 degrees to the right.
    fn turn_right(&self) -> Self {
        Self::new(self.coordinates.iter().map(|c| c.turn_right()))
    }

    /// Turns this piece 90 degrees to the left.
    fn turn_left(&self) -> Self {
        Self::new(self.coordinates.iter().map(|c| c.turn_left()))
    }

    /// Adjusts the coordinates of this piece shape to be relative
    /// to its minimum coords.
    fn align(&self) -> Self {
        let min_coords = self.coordinates.iter().fold(Coordinates::new(BOARD_SIZE, BOARD_SIZE), |(m, c)| m.min(c));
        Self::new(self.coordinates.iter().map(|c| c - min_coords))
    }

    /// Performs a rotation of this piece shape.
    pub fn rotate(&self, rotation: Rotation) -> Self {
        match rotation {
            Rotation::None => self,
            Rotation::Mirror => mirror().align(),
            Rotation::Right => turn_right().align(),
            Rotation::Left => turn_left().align()
        }
    }
}

// Operator overloads

impl Neg for Coordinates {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

impl Add for Coordinates {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Coordinates {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

// Conversions

impl FromStr for Team {
    type Err = SCError;

    fn from_str(raw: &str) -> SCResult<Self> {
        match raw.to_uppercase().as_str() {
            "NONE" => Ok(Self::None),
            "ONE" => Ok(Self::One),
            "TWO" => Ok(Self::Two),
            _ => Err(format!("Could not parse team {}", raw).into())
        }
    }
}

impl From<Team> for String {
    fn from(team: Team) -> Self {
        match team {
            Team::None => "NONE",
            Team::One => "ONE",
            Team::Two => "TWO"
        }
    }
}

impl TryFrom<i32> for Rotation {
    type Err = SCError;

    fn try_from(n: i32) -> SCResult<Self> {
        match n {
            0 => Ok(Self::None),
            1 => Ok(Self::Right),
            2 => Ok(Self::Mirror),
            3 => Ok(Self::Left),
            _ => Err(format!("Could not parse rotation {}", n).into())
        }
    }
}

impl From<Rotation> for i32 {
    fn from(rotation: Rotation) -> Self {
        match rotation {
            Rotation::None => 0,
            Rotation::Right => 1,
            Rotation::Mirror => 2,
            Rotation::Left => 3
        }
    }
}

impl TryFrom<usize> for PieceShape {
    type Err = SCError;

    fn try_from(kind: usize) -> SCResult<Self> {
        if kind >= 0 && kind < PIECE_SHAPES.len() {
            Ok(PIECE_SHAPES[kind])
        } else {
            Err(format!("Could not parse kind {} as piece shape", kind).into())
        }
    }
}
