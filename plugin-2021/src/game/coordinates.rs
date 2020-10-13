use std::ops::{Add, Neg, Sub};

use socha_client_base::{util::SCResult, xml_node::{FromXmlNode, XmlNode}};

/// A point in 2D-space. The x-axis
/// usually points to the right while
/// the y-axis points downwards.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32
}

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

    /// Flips the coordinates along the y-axis.
    pub fn flip(self) -> Self {
        Self::new(-self.x, self.y)
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

impl FromXmlNode for Coordinates {
    fn from_node(node: &XmlNode) -> SCResult<Self> {
        Ok(Self {
            x: node.attribute("x")?.parse()?,
            y: node.attribute("y")?.parse()?
        })
    }
}
