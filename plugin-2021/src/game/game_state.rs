use std::collections::{HashMap, HashSet};

use socha_client_base::{util::SCResult, xml_node::{FromXmlNode, XmlNode}};

use super::{Board, Color, Move, PIECE_SHAPES, Piece, PieceShape, Player, Team};

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
    pub blue_shapes: HashSet<PieceShape>,
    /// The undeployed yellow shapes.
    pub yellow_shapes: HashSet<PieceShape>,
    /// The undeployed red shapes.
    pub red_shapes: HashSet<PieceShape>,
    /// The undeployed green shapes.
    pub green_shapes: HashSet<PieceShape>
}

const SUM_MAX_SQUARES: i32 = 89;

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

    // Game rule logic is mostly a direct translation of
    // https://github.com/CAU-Kiel-Tech-Inf/backend/blob/97d185660754ffba4bd4444f3f39ae350f1d053e/plugin/src/shared/sc/plugin2021/util/GameRuleLogic.kt

    /// Computes the points from the given, undeployed piece shapes.
    fn get_points_from_undeployed(undeployed: HashSet<PieceShape>, mono_last: bool) -> i32 {
        // If all pieces were placed
        if undeployed.is_empty() {
            // Return sum of all squares plus 15 bonus points.
            // If the Monomino was the last placed piece, add another 5 points
            SUM_MAX_SQUARES + 15 + if mono_last { 5 } else { 0 }
        } else {
            // One point per piece placed
            let placed_points: i32 = undeployed.iter().map(|p| p.coordinates().count() as i32).sum();
            SUM_MAX_SQUARES - placed_points
        }
    }

    /// Whether the game state is in the first round.
    pub fn is_first_move(&self) -> bool {
        self.shapes_of_color(self.current_color()).count() == PIECE_SHAPES.len()
    }

    /// Performs the given move.
    pub fn perform_move(&mut self, game_move: Move) -> SCResult<()> {
        #[cfg(debug_assertions)]
        self.validate_move_color(&game_move)?;

        match game_move {
            Move::Set { piece } => self.perform_set_move(piece),
            Move::Skip { .. } => self.perform_skip_move()
        }
    }

    /// Fetches the state after the given move.
    pub fn after_move(&self, game_move: Move) -> SCResult<GameState> {
        let mut s = self.clone();
        s.perform_move(game_move)?;
        Ok(s)
    }

    /// Checks whether the given move has the right color.
    fn validate_move_color(&self, game_move: &Move) -> SCResult<()> {
        if game_move.color() != self.current_color() {
            Err(format!("Move color {} does not match game state color {}!", game_move.color(), self.current_color()).into())
        } else {
            Ok(())
        }
    }

    /// Checks whether the given shape is valid.
    fn validate_shape(&self, shape: &PieceShape, color: Color) -> SCResult<()> {
        if self.is_first_move() {
            if shape != &self.start_piece {
                return Err(format!("{} is not the (requested) first shape", shape).into())
            }
        } else if !self.shapes_of_color(color).any(|p| p == shape) {
            return Err(format!("Piece {} has already been placed before!", shape).into())
        }

        Ok(())
    }

    /// Checks whether the given set move is valid.
    fn validate_set_move(&self, piece: &Piece) -> SCResult<()> {
        self.validate_shape(&piece.shape(), piece.color)?;

        for coordinates in piece.coordinates() {
            if !self.board.is_in_bounds(coordinates) {
                return Err(format!("Target position of the set move {} is not in the board's bounds!", coordinates).into());
            }

            if self.board.is_obstructed(coordinates) {
                return Err(format!("Target position of the set move {} is obstructed!", coordinates).into());
            }

            if self.board.borders_on_color(coordinates, piece.color) {
                return Err(format!("Target position of the set move {} already borders on {}!", coordinates, piece.color).into());
            }
        }

        if self.is_first_move() {
            // Check whether it is placed correctly in a corner
            if !piece.coordinates().any(|p| self.board.is_on_corner(p)) {
                return Err("The piece from the set move is not located in a corner!".into());
            }
        } else {
            // Check whether the piece is connected to at least one tile of the same color by corner
            if !piece.coordinates().any(|p| self.board.corners_on_color(p, piece.color)) {
                return Err(format!("The piece {:?} shares no corner with another piece of same color!", piece).into());
            }
        }

        Ok(())
    }

    /// Performs the given set move.
    fn perform_set_move(&mut self, piece: Piece) -> SCResult<()> {
        #[cfg(debug_assertions)]
        self.validate_set_move(&piece)?;

        // TODO

        Ok(())
    }

    /// Performs the given skip move
    fn perform_skip_move(&self) -> SCResult<()> {
        // TODO

        Ok(())
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
