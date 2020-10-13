use socha_client_base::{error::SCError, hashmap, util::{SCResult, HasOpponent}, xml_node::FromXmlNode, xml_node::XmlNode};
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
    pub blue_shapes: HashSet<PieceShape>,
    pub yellow_shapes: HashSet<PieceShape>,
    pub red_shapes: HashSet<PieceShape>,
    pub green_shapes: HashSet<PieceShape>
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
pub struct Move {
    pub piece: Piece,
    /// The coordinates the upper left corner this piece is placed on.
    pub position: Coordinates
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Piece {
    pub kind: usize,
    pub rotation: Rotation,
    pub color: Color
}

/// Represents a shape in Blokus. There are 21 different kinds of these.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PieceShape {
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
            Self::Red => Self::Blue,
            Self::Blue => Self::Red
        }
    }
}

// Constants

lazy_static! {
    pub static ref BOARD_SIZE: usize = 20;
    pub static ref PIECE_SHAPES: HashMap<String, PieceShape> = hashmap![
        "MONO"    => PieceShape::new(&[Coordinates::new(0, 0)]),
        "DOMINO"  => PieceShape::new(&[Coordinates::new(0, 0), Coordinates::new(1, 0)]),
        "TRIO_L"  => PieceShape::new(&[Coordinates::new(0, 0), Coordinates::new(0, 1), Coordinates::new(1, 1)]),
        "TRIO_I"  => PieceShape::new(&[Coordinates::new(0, 0), Coordinates::new(0, 1), Coordinates::new(0, 2)]),
        "TETRO_O" => PieceShape::new(&[Coordinates::new(0, 0), Coordinates::new(1, 0), Coordinates::new(0, 1), Coordinates::new(1, 1)]),
        "TETRO_T" => PieceShape::new(&[Coordinates::new(0, 0), Coordinates::new(1, 0), Coordinates::new(2, 0), Coordinates::new(1, 1)]),
        "TETRO_I" => PieceShape::new(&[Coordinates::new(0, 0), Coordinates::new(0, 1), Coordinates::new(0, 2), Coordinates::new(0, 3)]),
        "TETRO_L" => PieceShape::new(&[Coordinates::new(0, 0), Coordinates::new(0, 1), Coordinates::new(0, 2), Coordinates::new(1, 2)]),
        "TETRO_Z" => PieceShape::new(&[Coordinates::new(0, 0), Coordinates::new(1, 0), Coordinates::new(1, 1), Coordinates::new(2, 1)]),
        "PENTO_L" => PieceShape::new(&[Coordinates::new(0, 0), Coordinates::new(0, 1), Coordinates::new(0, 2), Coordinates::new(0, 3), Coordinates::new(1, 3)]),
        "PENTO_T" => PieceShape::new(&[Coordinates::new(0, 0), Coordinates::new(1, 0), Coordinates::new(2, 0), Coordinates::new(1, 1), Coordinates::new(1, 2)]),
        "PENTO_V" => PieceShape::new(&[Coordinates::new(0, 0), Coordinates::new(0, 1), Coordinates::new(0, 2), Coordinates::new(1, 2), Coordinates::new(2, 2)]),
        "PENTO_S" => PieceShape::new(&[Coordinates::new(1, 0), Coordinates::new(2, 0), Coordinates::new(3, 0), Coordinates::new(0, 1), Coordinates::new(1, 1)]),
        "PENTO_Z" => PieceShape::new(&[Coordinates::new(0, 0), Coordinates::new(1, 0), Coordinates::new(1, 1), Coordinates::new(1, 2), Coordinates::new(2, 2)]),
        "PENTO_I" => PieceShape::new(&[Coordinates::new(0, 0), Coordinates::new(0, 1), Coordinates::new(0, 2), Coordinates::new(0, 3), Coordinates::new(0, 4)]),
        "PENTO_P" => PieceShape::new(&[Coordinates::new(0, 0), Coordinates::new(1, 0), Coordinates::new(0, 1), Coordinates::new(1, 1), Coordinates::new(0, 2)]),
        "PENTO_W" => PieceShape::new(&[Coordinates::new(0, 0), Coordinates::new(0, 1), Coordinates::new(1, 1), Coordinates::new(1, 2), Coordinates::new(2, 2)]),
        "PENTO_U" => PieceShape::new(&[Coordinates::new(0, 0), Coordinates::new(0, 1), Coordinates::new(1, 1), Coordinates::new(2, 1), Coordinates::new(2, 0)]),
        "PENTO_R" => PieceShape::new(&[Coordinates::new(0, 1), Coordinates::new(1, 1), Coordinates::new(1, 2), Coordinates::new(2, 1), Coordinates::new(2, 0)]),
        "PENTO_X" => PieceShape::new(&[Coordinates::new(1, 0), Coordinates::new(0, 1), Coordinates::new(1, 1), Coordinates::new(2, 1), Coordinates::new(1, 2)]),
        "PENTO_Y" => PieceShape::new(&[Coordinates::new(0, 1), Coordinates::new(1, 0), Coordinates::new(1, 1), Coordinates::new(1, 2), Coordinates::new(1, 3)])
    ];
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

impl Piece {
    pub fn new(kind: usize, rotation: Rotation, color: Color) -> Self {
        Self { kind, rotation, color }
    }
}

impl PieceShape {
    fn new(coordinates: impl IntoIterator<Item=Coordinates>) -> Self {
        Self { coordinates: coordinates.into_iter().collect() }
    }

    /// A list of occupied fields, with the upper left corner being
    /// the origin (0, 0), the x-axis pointed right and the y-axis pointed downwards
    pub fn coordinates(&self) -> &HashSet<Coordinates> {
        self.coordinates
    }

    /// Mirrors this shape by negating all coordinates.
    fn mirror(&self) -> Self {
        Self::new(self.coordinates.iter().map(|c| -c))
    }

    /// Turns this piece 90 degrees to the right.
    fn turn_right(&self) -> Self {
        Self::new(self.coordinates.iter().map(|c| c.turn_right()))
    }

    /// Turns this piece 90 degrees to the left.
    fn turn_left(&self) -> Self {
        Self::new(self.coordinates.iter().map(|c| c.turn_left()))
    }

    /// Adjusts the coordinates of this piece shape to be relative
    /// to its minimum coords.
    fn align(&self) -> Self {
        let min_coords = self.coordinates.iter().fold(Coordinates::new(BOARD_SIZE, BOARD_SIZE), |(m, c)| m.min(c));
        Self::new(self.coordinates.iter().map(|c| c - min_coords))
    }

    /// Performs a rotation of this piece shape.
    pub fn rotate(&self, rotation: Rotation) -> Self {
        match rotation {
            Rotation::None => self,
            Rotation::Mirror => self.mirror().align(),
            Rotation::Right => self.turn_right().align(),
            Rotation::Left => self.turn_left().align()
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
            "BLUE" => Ok(Color::Blue),
            "YELLOW" => Ok(Color::Yellow),
            "RED" => Ok(Color::Red),
            "GREEN" => Ok(Color::Green),
            _ => Err(format!("Color not parse color {}", raw).into())
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Team::Blue => write!(f, "BLUE"),
            Team::Yellow => write!(f, "YELLOW"),
            Team::Red => write!(f, "RED"),
            Team::Green => write!(f, "GREEN")
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

impl FromStr for PieceShape {
    type Err = SCError;

    fn from_str(raw: &str) -> SCResult<Self> {
        PIECE_SHAPES.get(raw).ok_or_else(|| format!("Could not parse shape {}", raw).into())
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
            ordered_colors: node.child_by_name("orderedColors")?.childs_by_name("color").map(Color::from_node),
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
            display_name: node.attribute("displayName")?
        })
    }
}
