use serde::{Serialize, Deserialize};

/// A message indicating that the client
/// has joined a room with the specified id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "joined", rename_all = "camelCase")]
pub struct Joined {
    pub room_id: String
}
