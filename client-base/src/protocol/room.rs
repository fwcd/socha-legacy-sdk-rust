use serde::{Serialize, Deserialize};
use crate::plugin::SCPlugin;
use super::Data;

/// A message in a room together with some data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "room", rename_all = "camelCase")]
pub struct Room<P> where P: SCPlugin {
    pub room_id: String,
    #[serde(bound(
        serialize = "P::Player: Serialize, P::Move: Serialize, P::PlayerColor: Serialize, P::GameState: Serialize",
        deserialize = "P::Player: Deserialize<'de>, P::Move: Deserialize<'de>, P::PlayerColor: Deserialize<'de>, P::GameState: Deserialize<'de>"
    ))]
    pub data: Data<P>
}
