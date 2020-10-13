use std::{fmt, ops::{Add, Neg, Sub}};

use socha_client_base::{util::SCResult, xml_node::{FromXmlNode, XmlNode}};

/// A vector in 2D-space. The x-axis
/// usually points to the right while
/// the y-axis points downwards.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32
}

impl Vec2 {
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

    /// Flips the coordinates along the y-axis.
    pub fn flip(self) -> Self {
        Self::new(-self.x, self.y)
    }

    /// Finds the minimum with another point.
    pub fn min(self, other: Vec2) -> Self {
        Self::new(self.x.min(other.x), self.y.min(other.y))
    }

    /// Finds the maximum with another point.
    pub fn max(self, other: Vec2) -> Self {
        Self::new(self.x.max(other.x), self.y.max(other.y))
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl FromXmlNode for Vec2 {
    fn from_node(node: &XmlNode) -> SCResult<Self> {
        Ok(Self {
            x: node.attribute("x")?.parse()?,
            y: node.attribute("y")?.parse()?
        })
    }
}
