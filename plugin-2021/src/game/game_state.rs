use std::collections::HashMap;

use socha_client_base::{util::SCResult, xml_node::{FromXmlNode, XmlNode}};

use super::{Board, Color, PieceShape, Player, Team};

/// A snapshot of the game's state. It holds the
/// information needed to compute the next move.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    /// The number of already committed moves.
    pub turn: u32,
    /// The number of rounds.
    pub round: u32,
    /// The first team's player.
    pub first: Player,
    /// The second team's player.
    pub second: Player,
    /// The current game board.
    pub board: Board,
    /// The piece that has to be placed in the first round.
    pub start_piece: PieceShape,
    /// The color that begins the game.
    pub start_color: Color,
    /// The team that begins the game.
    pub start_team: Team,
    /// A list of all colors currently in the game.
    pub ordered_colors: Vec<Color>,
    /// A map that stores, for each color, whether the last move was a monomino if all pieces have been placed.
    pub last_move_mono: HashMap<Color, bool>,
    /// The current color's index
    pub current_color_index: u32,
    /// The undeployed blue shapes.
    pub blue_shapes: Vec<PieceShape>,
    /// The undeployed yellow shapes.
    pub yellow_shapes: Vec<PieceShape>,
    /// The undeployed red shapes.
    pub red_shapes: Vec<PieceShape>,
    /// The undeployed green shapes.
    pub green_shapes: Vec<PieceShape>
}

impl GameState {
    /// Fetches the current color.
    pub fn current_color(&self) -> Color {
        self.ordered_colors[self.current_color_index as usize]
    }

    /// Fetches the current team.
    pub fn current_team(&self) -> Team {
        self.current_color().team()
    }

    /// Fetches the current player.
    pub fn current_player(&self) -> &Player {
        match self.current_team() {
            Team::One => &self.first,
            Team::Two => &self.second,
            Team::None => panic!("Cannot fetch the current player with the team being 'none'!")
        }
    }

    /// Fetches the undeployed piece shapes of a given color.
    pub fn shapes_of_color(&self, color: Color) -> impl Iterator<Item=&PieceShape> {
        match color {
            Color::Red => self.red_shapes.iter(),
            Color::Yellow => self.yellow_shapes.iter(),
            Color::Green => self.green_shapes.iter(),
            Color::Blue => self.blue_shapes.iter(),
            Color::None => panic!("Cannot fetch shapes of color 'none'!")
        }
    }
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
