use serde::{Serialize, Deserialize};
use crate::plugin::SCPlugin;
use super::{Room, Data, Joined, Left, Close};

/// A generic message packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Packet<P> where P: SCPlugin {
    Room(Room<P>),
    Data(Data<P>),
    Joined(Joined),
    Left(Left),
    Close(Close)
}
