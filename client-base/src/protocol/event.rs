use serde::{Serialize, Deserialize};
use crate::{plugin::SCPlugin};
use super::GameResult;

/// A polymorphic container for game data
/// used by the protocol. It is parameterized
/// by the player color (`C`), the game state (`S`)
/// and the player structure (`P`). These types
/// are implemented independently of the base
/// protocol for each year's game.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "data", tag = "class", rename_all = "camelCase")]
pub enum Event<P> where P: SCPlugin {
    WelcomeMessage { color: P::PlayerColor },
    Memento { state: P::GameState },
    #[serde(rename = "sc.framework.plugins.protocol.MoveRequest")]
    MoveRequest,
    Move { r#move: P::Move },
    #[serde(bound(serialize = "P::Player: Serialize", deserialize = "P::Player: Deserialize<'de>"))]
    Result(GameResult<P::Player>),
    Error { message: String },
}
