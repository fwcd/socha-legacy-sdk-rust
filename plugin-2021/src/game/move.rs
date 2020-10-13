use socha_client_base::xml_node::XmlNode;

use super::Piece;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Move {
    /// A move that skips a round.
    Skip,
    /// A move that places an own, not yet placed piece.
    Set { piece: Piece }
}

impl From<Move> for XmlNode {
    fn from(game_move: Move) -> Self {
        match game_move {
            Move::Set { piece } => XmlNode::new("data")
                .attribute("class", "sc.plugin2021.SetMove")
                .child(piece)
                .build(),
            Move::Skip => XmlNode::new("data")
                .attribute("class", "sc.plugin2021.SkipMove")
                .build()
        }
    }
}
