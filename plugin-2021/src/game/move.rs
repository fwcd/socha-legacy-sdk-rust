use serde::{Serialize, Deserialize};
use socha_client_base::util::serde_as_wrapped_value;

use super::{Color, Piece};

/// A move in the game.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "data", tag = "class")]
pub enum Move {
    /// A move that skips a round.
    #[serde(rename = "sc.plugin2021.SkipMove")]
    Skip {
        #[serde(with = "serde_as_wrapped_value")]
        color: Color
    },
    /// A move that places an own, not yet placed piece.
    #[serde(rename = "sc.plugin2021.SetMove")]
    Set {
        piece: Piece
    }
}

impl Move {
    pub fn color(&self) -> Color {
        match self {
            Self::Skip { color } => *color,
            Self::Set { piece } => piece.color
        }
    }
}
