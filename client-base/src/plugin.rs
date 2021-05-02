use std::str::FromStr;
use std::fmt::Debug;
use crate::util::HasOpponent;
use crate::error::SCError;

/// A "type family" trait that defines types
/// which represent various parts of a game.
pub trait SCPlugin: Debug {
    type PlayerColor: Copy + Debug + Eq + HasOpponent + FromStr<Err=SCError>;
    type Player: Clone + Debug + Eq;
    type GameState: Clone + Debug + Eq + HasPlayerColor<PlayerColor=Self::PlayerColor> + HasTurn;
    type Move: Clone + Debug + Eq;
    
    /// Fetches the 'gameType' used during
    /// the protocol handshake.
    fn protocol_game_type<'a>() -> &'a str;
}

/// Indicates that the value has an "associated" player color.
/// The plugin-specific `GameState` should return the current player
/// color when implementing this trait.
pub trait HasPlayerColor {
    type PlayerColor;

    /// Fetches the associated player color.
    fn player_color(&self) -> Self::PlayerColor;
}

/// Indicates that the value has a turn.
pub trait HasTurn {
    /// Fetches the turn.
    fn turn(&self) -> u32;
}

/// A mock implementation of a plugin for testing.
#[derive(Debug)]
pub struct MockPlugin;

/// A mock player color for testing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MockPlayerColor {
    Red,
    Blue,
}

/// A mock player for testing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MockPlayer;

/// A mock game state for testing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MockGameState {
    player_color: MockPlayerColor,
    turn: u32,
}

/// A mock move for testing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MockMove;

impl SCPlugin for MockPlugin {
    type PlayerColor = MockPlayerColor;
    type Player = MockPlayer;
    type GameState = MockGameState;
    type Move = MockMove;

    fn protocol_game_type<'a>() -> &'a str { "mock" }
}

impl FromStr for MockPlayerColor {
    type Err = SCError;

    fn from_str(s: &str) -> Result<Self, SCError> {
        match s {
            "RED" => Ok(Self::Red),
            "BLUE" => Ok(Self::Blue),
            _ => Err(SCError::Custom(format!("Invalid mock player color: {}", s))),
        }
    }
}

impl HasOpponent for MockPlayerColor {
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

impl HasPlayerColor for MockGameState {
    type PlayerColor = MockPlayerColor;

    fn player_color(&self) -> MockPlayerColor {
        self.player_color
    }
}
