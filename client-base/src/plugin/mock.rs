use serde::{Serialize, Deserialize};
use std::{fmt, str::FromStr};

use crate::error::SCError;
use crate::util::{serde_as_str, serde_as_wrapped_value};

use super::{SCPlugin, HasTeam, HasOpponent, HasTurn};

/// A mock implementation of a plugin for testing.
#[derive(Debug, PartialEq, Eq)]
pub struct MockPlugin;

/// A mock team for testing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MockTeam {
    Red,
    Blue,
}

/// A mock player for testing.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MockPlayer {
    #[serde(with = "serde_as_str")]
    pub team: MockTeam,
    pub display_name: String,
}

/// A mock game state for testing.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MockGameState {
    #[serde(with = "serde_as_wrapped_value")]
    pub team: MockTeam,
    pub turn: u32,
}

/// A mock move for testing.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MockMove;

impl SCPlugin for MockPlugin {
    type Team = MockTeam;
    type Player = MockPlayer;
    type GameState = MockGameState;
    type Move = MockMove;

    fn protocol_game_type<'a>() -> &'a str { "mock" }
}

impl FromStr for MockTeam {
    type Err = SCError;

    fn from_str(s: &str) -> Result<Self, SCError> {
        match s {
            "RED" => Ok(Self::Red),
            "BLUE" => Ok(Self::Blue),
            _ => Err(SCError::Custom(format!("Invalid mock player color: {}", s))),
        }
    }
}

impl fmt::Display for MockTeam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Red => write!(f, "RED"),
            Self::Blue => write!(f, "BLUE"),
        }
    }
}

impl HasOpponent for MockTeam {
    fn opponent(self) -> Self {
        match self {
            Self::Red => Self::Blue,
            Self::Blue => Self::Red,
        }
    }
}

impl HasTurn for MockGameState {
    fn turn(&self) -> u32 {
        self.turn
    }
}

impl HasTeam for MockGameState {
    type Team = MockTeam;

    fn team(&self) -> MockTeam {
        self.team
    }
}
