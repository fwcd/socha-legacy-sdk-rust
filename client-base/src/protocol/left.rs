use serde::{Serialize, Deserialize};

/// A message indicating that the client
/// has left a room with the specified id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "left", rename_all = "camelCase")]
pub struct Left {
    pub room_id: String
}
