use socha_client_base::{util::SCResult, xml_node::FromXmlNode, xml_node::XmlNode};

use super::{Color, Coordinates, PieceShape, Rotation};

/// A game piece with color, position and transformed form.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Piece {
    /// The piece's untransformed shape
    pub kind: PieceShape,
    /// How far the piece has been rotated
    pub rotation: Rotation,
    /// Whether the piece has been mirrored along the y-axis
    pub is_flipped: bool,
    /// The piece's color
    pub color: Color,
    /// The top left corner of the piece's rectangular bounding box
    pub position: Coordinates
}

impl Piece {
    /// Fetches the piece's actual (transformed) shape
    pub fn shape(&self) -> PieceShape {
        let mut shape = self.kind.rotate(self.rotation);
        if self.is_flipped {
            shape = shape.flip();
        }
        shape
    }

    /// Fetches the piece's actual coordinates.
    pub fn coordinates(&self) -> impl Iterator<Item=Coordinates> {
        let position = self.position;
        self.shape().coordinates().map(move |c| c + position)
    }
}

impl FromXmlNode for Piece {
    fn from_node(node: &XmlNode) -> SCResult<Self> {
        Ok(Self {
            color: node.attribute("color")?.parse()?,
            kind: node.attribute("kind")?.parse()?,
            rotation: node.attribute("rotation")?.parse()?,
            is_flipped: node.attribute("isFlipped")?.parse()?,
            position: Coordinates::from_node(node.child_by_name("position")?)?
        })
    }
}

impl From<Piece> for XmlNode {
    fn from(piece: Piece) -> Self {
        XmlNode::new("piece")
            .attribute("color", piece.color.to_string())
            .attribute("kind", piece.kind.to_string())
            .attribute("rotation", piece.rotation.to_string())
            .attribute("is_flipped", piece.is_flipped.to_string())
            .child(XmlNode::new("position")
                .attribute("x", piece.position.x.to_string())
                .attribute("y", piece.position.y.to_string())
                .build())
            .build()
    }
}
