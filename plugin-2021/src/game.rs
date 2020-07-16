use socha_client_base::{error::SCError, util::{SCResult, HasOpponent}};
use std::{str::FromStr, convert::TryFrom};

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
    pub kind: i32,
    pub rotation: Rotation,
    pub color: Color
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rotation {
    None,
    Right,
    Mirror,
    Left
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

// General conversions

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

    fn from(n: i32) -> SCResult<Self> {
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
