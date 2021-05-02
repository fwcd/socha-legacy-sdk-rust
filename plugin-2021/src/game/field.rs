use serde::{Serialize, Deserialize};
use super::{Color, Vec2};

/// A field on the board holding a color.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Field {
    // Unpacked position
    x: i32,
    y: i32,
    pub content: Color
}

impl Field {
    pub fn new(position: Vec2, content: Color) -> Self {
        Self { x: position.x, y: position.y, content }
    }

    pub fn position(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}
