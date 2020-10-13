use socha_client_base::{error::SCError, util::{SCResult, HasOpponent}, xml_node::FromXmlNode, xml_node::XmlNode};
use std::{collections::{HashSet, HashMap}, convert::TryFrom, fmt, ops::{Add, Sub, Neg}, str::FromStr};
use lazy_static::lazy_static;

// Structures

/// A color in the game.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Color {
    None,
    Blue,
    Yellow,
    Red,
    Green
}

/// A player's team.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Team {
    None,
    One,
    Two
}

/// Metadata about a player.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Player {
    pub team: Team,
    pub display_name: String
}

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    // TODO: More efficient representation, e.g. using a 2D matrix of colors
    fields: Vec<Field>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    pub x: u32,
    pub y: u32,
    pub content: Color
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Move {
    /// A move that skips a round.
    Skip,
    /// A move that places an own, not yet placed piece.
    Set { piece: Piece }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Piece {
    /// The piece's shape
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

/// Represents a shape in Blokus. There are 21 different kinds of these.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PieceShape {
    name: String,
    coordinates: HashSet<Coordinates>
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Rotation {
    None,
    Right,
    Mirror,
    Left
}

/// A point in 2D-space. The x-axis
/// usually points to the right while
/// the y-axis points downwards.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32
}

impl HasOpponent for Team {
    fn opponent(self) -> Self {
        match self {
            Self::None => Self::None,
            Self::One => Self::Two,
            Self::Two => Self::One
        }
    }
}

// Constants

pub const BOARD_SIZE: usize = 20;

lazy_static! {
    pub static ref PIECE_SHAPES: [PieceShape; 21] = [
        PieceShape::new("MONO", vec![Coordinates::new(0, 0)]),
        PieceShape::new("DOMINO", vec![Coordinates::new(0, 0), Coordinates::new(1, 0)]),
        PieceShape::new("TRIO_L", vec![Coordinates::new(0, 0), Coordinates::new(0, 1), Coordinates::new(1, 1)]),
        PieceShape::new("TRIO_I", vec![Coordinates::new(0, 0), Coordinates::new(0, 1), Coordinates::new(0, 2)]),
        PieceShape::new("TETRO_O", vec![Coordinates::new(0, 0), Coordinates::new(1, 0), Coordinates::new(0, 1), Coordinates::new(1, 1)]),
        PieceShape::new("TETRO_T", vec![Coordinates::new(0, 0), Coordinates::new(1, 0), Coordinates::new(2, 0), Coordinates::new(1, 1)]),
        PieceShape::new("TETRO_I", vec![Coordinates::new(0, 0), Coordinates::new(0, 1), Coordinates::new(0, 2), Coordinates::new(0, 3)]),
        PieceShape::new("TETRO_L", vec![Coordinates::new(0, 0), Coordinates::new(0, 1), Coordinates::new(0, 2), Coordinates::new(1, 2)]),
        PieceShape::new("TETRO_Z", vec![Coordinates::new(0, 0), Coordinates::new(1, 0), Coordinates::new(1, 1), Coordinates::new(2, 1)]),
        PieceShape::new("PENTO_L", vec![Coordinates::new(0, 0), Coordinates::new(0, 1), Coordinates::new(0, 2), Coordinates::new(0, 3), Coordinates::new(1, 3)]),
        PieceShape::new("PENTO_T", vec![Coordinates::new(0, 0), Coordinates::new(1, 0), Coordinates::new(2, 0), Coordinates::new(1, 1), Coordinates::new(1, 2)]),
        PieceShape::new("PENTO_V", vec![Coordinates::new(0, 0), Coordinates::new(0, 1), Coordinates::new(0, 2), Coordinates::new(1, 2), Coordinates::new(2, 2)]),
        PieceShape::new("PENTO_S", vec![Coordinates::new(1, 0), Coordinates::new(2, 0), Coordinates::new(3, 0), Coordinates::new(0, 1), Coordinates::new(1, 1)]),
        PieceShape::new("PENTO_Z", vec![Coordinates::new(0, 0), Coordinates::new(1, 0), Coordinates::new(1, 1), Coordinates::new(1, 2), Coordinates::new(2, 2)]),
        PieceShape::new("PENTO_I", vec![Coordinates::new(0, 0), Coordinates::new(0, 1), Coordinates::new(0, 2), Coordinates::new(0, 3), Coordinates::new(0, 4)]),
        PieceShape::new("PENTO_P", vec![Coordinates::new(0, 0), Coordinates::new(1, 0), Coordinates::new(0, 1), Coordinates::new(1, 1), Coordinates::new(0, 2)]),
        PieceShape::new("PENTO_W", vec![Coordinates::new(0, 0), Coordinates::new(0, 1), Coordinates::new(1, 1), Coordinates::new(1, 2), Coordinates::new(2, 2)]),
        PieceShape::new("PENTO_U", vec![Coordinates::new(0, 0), Coordinates::new(0, 1), Coordinates::new(1, 1), Coordinates::new(2, 1), Coordinates::new(2, 0)]),
        PieceShape::new("PENTO_R", vec![Coordinates::new(0, 1), Coordinates::new(1, 1), Coordinates::new(1, 2), Coordinates::new(2, 1), Coordinates::new(2, 0)]),
        PieceShape::new("PENTO_X", vec![Coordinates::new(1, 0), Coordinates::new(0, 1), Coordinates::new(1, 1), Coordinates::new(2, 1), Coordinates::new(1, 2)]),
        PieceShape::new("PENTO_Y", vec![Coordinates::new(0, 1), Coordinates::new(1, 0), Coordinates::new(1, 1), Coordinates::new(1, 2), Coordinates::new(1, 3)])
    ];

    pub static ref PIECE_SHAPES_BY_NAME: HashMap<String, PieceShape> = {
        let mut m = HashMap::new();
        for piece in PIECE_SHAPES.iter() {
            m.insert(piece.name.to_owned(), piece.clone());
        }
        m
    };
}

// Implementations

impl Coordinates {
    /// Creates new coordinates.
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Rotates these coordinates 90 degrees clockwise.
    pub fn turn_right(self) -> Self {
        Self::new(-self.y, self.x)
    }

    /// Rotates these coordinates 90 degrees counter-clockwise.
    pub fn turn_left(self) -> Self {
        Self::new(self.y, -self.x)
    }

    /// Finds the minimum with another point.
    pub fn min(self, other: Coordinates) -> Self {
        Self::new(self.x.min(other.x), self.y.min(other.y))
    }

    /// Finds the maximum with another point.
    pub fn max(self, other: Coordinates) -> Self {
        Self::new(self.x.max(other.x), self.y.max(other.y))
    }
}

impl PieceShape {
    fn new(name: &str, coordinates: impl IntoIterator<Item=Coordinates>) -> Self {
        Self { name: name.to_owned(), coordinates: coordinates.into_iter().collect() }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// A list of occupied fields, with the upper left corner being
    /// the origin (0, 0), the x-axis pointed right and the y-axis pointed downwards
    pub fn coordinates(&self) -> &HashSet<Coordinates> {
        &self.coordinates
    }

    /// Mirrors this shape by negating all coordinates.
    fn mirror(&self) -> Self {
        Self::new(self.name(), self.coordinates.iter().map(|&c| -c))
    }

    /// Turns this piece 90 degrees to the right.
    fn turn_right(&self) -> Self {
        Self::new(self.name(), self.coordinates.iter().map(|c| c.turn_right()))
    }

    /// Turns this piece 90 degrees to the left.
    fn turn_left(&self) -> Self {
        Self::new(self.name(), self.coordinates.iter().map(|c| c.turn_left()))
    }

    /// Adjusts the coordinates of this piece shape to be relative
    /// to its minimum coords.
    fn align(&self) -> Self {
        let min_coords = self.coordinates.iter().fold(Coordinates::new(BOARD_SIZE as i32, BOARD_SIZE as i32), |m, &c| m.min(c));
        Self::new(self.name(), self.coordinates.iter().map(|&c| c - min_coords))
    }

    /// Performs a rotation of this piece shape.
    pub fn rotate(&self, rotation: Rotation) -> Self {
        match rotation {
            Rotation::None => self.clone(),
            Rotation::Mirror => self.mirror().align(),
            Rotation::Right => self.turn_right().align(),
            Rotation::Left => self.turn_left().align()
        }
    }
}

impl Color {
    pub fn team(self) -> Team {
        match self {
            Color::Red | Color::Blue => Team::One,
            Color::Yellow | Color::Green => Team::Two,
            Color::None => Team::None
        }
    }
}

// Operator overloads

impl Neg for Coordinates {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

impl Add for Coordinates {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Coordinates {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

// Conversions

impl FromStr for Team {
    type Err = SCError;

    fn from_str(raw: &str) -> SCResult<Self> {
        match raw.to_uppercase().as_str() {
            "NONE" => Ok(Self::None),
            "ONE" => Ok(Self::One),
            "TWO" => Ok(Self::Two),
            _ => Err(format!("Could not parse team {}", raw).into())
        }
    }
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Team::None => write!(f, "NONE"),
            Team::One => write!(f, "ONE"),
            Team::Two => write!(f, "TWO")
        }
    }
}

impl FromStr for Color {
    type Err = SCError;

    fn from_str(raw: &str) -> SCResult<Self> {
        match raw.to_uppercase().as_str() {
            "BLUE" => Ok(Self::Blue),
            "YELLOW" => Ok(Self::Yellow),
            "RED" => Ok(Self::Red),
            "GREEN" => Ok(Self::Green),
            _ => Err(format!("Color not parse color {}", raw).into())
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Blue => write!(f, "BLUE"),
            Self::Yellow => write!(f, "YELLOW"),
            Self::Red => write!(f, "RED"),
            Self::Green => write!(f, "GREEN"),
            Self::None => write!(f, "NONE")
        }
    }
}

impl TryFrom<i32> for Rotation {
    type Error = SCError;

    fn try_from(n: i32) -> SCResult<Self> {
        match n {
            0 => Ok(Self::None),
            1 => Ok(Self::Right),
            2 => Ok(Self::Mirror),
            3 => Ok(Self::Left),
            _ => Err(format!("Could not parse rotation {}", n).into())
        }
    }
}

impl From<Rotation> for i32 {
    fn from(rotation: Rotation) -> Self {
        match rotation {
            Rotation::None => 0,
            Rotation::Right => 1,
            Rotation::Mirror => 2,
            Rotation::Left => 3
        }
    }
}

impl FromStr for Rotation {
    type Err = SCError;

    fn from_str(raw: &str) -> SCResult<Self> {
        match raw.to_uppercase().as_str() {
            "NONE" => Ok(Rotation::None),
            "RIGHT" => Ok(Rotation::Right),
            "MIRROR" => Ok(Rotation::Mirror),
            "LEFT" => Ok(Rotation::Left),
            _ => Err(format!("Could not parse rotation {}", raw).into())
        }
    }
}

impl fmt::Display for Rotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rotation::None => write!(f, "NONE"),
            Rotation::Right => write!(f, "RIGHT"),
            Rotation::Mirror => write!(f, "MIRROR"),
            Rotation::Left => write!(f, "LEFT")
        }
    }
}

impl FromStr for PieceShape {
    type Err = SCError;

    fn from_str(raw: &str) -> SCResult<Self> {
        Ok(PIECE_SHAPES_BY_NAME.get(raw).ok_or_else(|| format!("Could not parse shape {}", raw))?.clone())
    }
}

impl fmt::Display for PieceShape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

// XML conversions

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

impl FromXmlNode for Board {
    fn from_node(node: &XmlNode) -> SCResult<Self> {
        Ok(Self {
            fields: node.childs_by_name("field").map(Field::from_node).collect::<Result<_, _>>()?
        })
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

impl FromXmlNode for Coordinates {
    fn from_node(node: &XmlNode) -> SCResult<Self> {
        Ok(Self {
            x: node.attribute("x")?.parse()?,
            y: node.attribute("y")?.parse()?
        })
    }
}

impl FromXmlNode for Field {
    fn from_node(node: &XmlNode) -> SCResult<Self> {
        Ok(Self {
            x: node.attribute("x")?.parse()?,
            y: node.attribute("y")?.parse()?,
            content: node.attribute("content")?.parse()?
        })
    }
}

impl FromXmlNode for PieceShape {
    fn from_node(node: &XmlNode) -> SCResult<Self> {
        node.content().parse()
    }
}

impl FromXmlNode for Color {
    fn from_node(node: &XmlNode) -> SCResult<Self> {
        node.content().parse()
    }
}

impl FromXmlNode for Team {
    fn from_node(node: &XmlNode) -> SCResult<Self> {
        node.content().parse()
    }
}

impl FromXmlNode for Player {
    fn from_node(node: &XmlNode) -> SCResult<Self> {
        Ok(Self {
            team: Team::from_node(node.child_by_name("color")?)?,
            display_name: node.attribute("displayName")?.to_owned()
        })
    }
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
