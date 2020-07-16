use socha_client_base::{error::SCError, util::HasOpponent};

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
