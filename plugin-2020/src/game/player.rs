use serde::{Serialize, Deserialize};
use super::PlayerColor;

/// Metadata about a player.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub color: PlayerColor,
    pub display_name: String
}
