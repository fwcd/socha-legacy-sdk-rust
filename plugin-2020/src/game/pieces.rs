use serde::{Serialize, Deserialize};
use super::Piece;

/// Pieces of the board. This is a separate struct
/// due to the way it is represented in XML.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pieces {
    #[serde(rename = "piece")]
    pieces: Vec<Piece>
}

impl Pieces {
    pub fn new() -> Self {
        Self { pieces: Vec::new() }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.pieces.len()
    }

    #[inline]
    pub fn contains(&self, piece: &Piece) -> bool {
        self.pieces.contains(piece)
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item=&Piece> {
        self.pieces.iter()
    }
}
