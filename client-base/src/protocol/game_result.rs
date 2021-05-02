use serde::{Serialize, Deserialize};
use super::{PlayerScore, ScoreDefinition};

/// The final result of a game.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameResult<P> {
    pub definition: ScoreDefinition,
    #[serde(rename = "score")]
    pub scores: Vec<PlayerScore>,
    #[serde(rename = "winner")]
    pub winners: Vec<P>,
}
