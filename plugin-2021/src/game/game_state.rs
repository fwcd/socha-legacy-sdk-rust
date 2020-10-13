use std::collections::HashMap;

use socha_client_base::{util::SCResult, xml_node::{FromXmlNode, XmlNode}};

use super::{Board, Color, PieceShape, Player, Team};

/// A snapshot of the game's state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    pub turn: u32,
    pub round: u32,
    pub first: Player,
    pub second: Player,
    pub board: Board,
    pub start_piece: PieceShape,
    pub start_color: Color,
    pub start_team: Team,
    // TODO: Current team accessor
    pub ordered_colors: Vec<Color>,
    pub last_move_mono: HashMap<Color, bool>,
    pub current_color_index: u32,
    // TODO: Accessor for color -> piece shape
    pub blue_shapes: Vec<PieceShape>,
    pub yellow_shapes: Vec<PieceShape>,
    pub red_shapes: Vec<PieceShape>,
    pub green_shapes: Vec<PieceShape>
}

impl FromXmlNode for GameState {
    fn from_node(node: &XmlNode) -> SCResult<Self> {
        Ok(Self {
            turn: node.attribute("turn")?.parse()?,
            round: node.attribute("round")?.parse()?,
            first: Player::from_node(node.child_by_name("first")?)?,
            second: Player::from_node(node.child_by_name("second")?)?,
            board: Board::from_node(node.child_by_name("board")?)?,
            start_piece: node.attribute("startPiece")?.parse()?,
            start_color: Color::from_node(node.child_by_name("startColor")?)?,
            start_team: Team::from_node(node.child_by_name("startTeam")?)?,
            ordered_colors: node.child_by_name("orderedColors")?.childs_by_name("color").map(Color::from_node).collect::<Result<_, _>>()?,
            last_move_mono: HashMap::new(), // TODO
            current_color_index: node.attribute("currentColorIndex")?.parse()?,
            blue_shapes: node.child_by_name("blueShapes")?.childs_by_name("shape").map(PieceShape::from_node).collect::<Result<_, _>>()?,
            yellow_shapes: node.child_by_name("yellowShapes")?.childs_by_name("shape").map(PieceShape::from_node).collect::<Result<_, _>>()?,
            red_shapes: node.child_by_name("redShapes")?.childs_by_name("shape").map(PieceShape::from_node).collect::<Result<_, _>>()?,
            green_shapes: node.child_by_name("greenShapes")?.childs_by_name("shape").map(PieceShape::from_node).collect::<Result<_, _>>()?
        })
    }
}
