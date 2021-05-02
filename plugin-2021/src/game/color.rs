use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};
use std::{convert::TryFrom, fmt, str::FromStr};

use socha_client_base::{error::SCError, util::SCResult};

use super::Team;

/// A color in the game.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Color {
    None,
    Blue,
    Yellow,
    Red,
    Green
}

impl Color {
    /// Unwraps an Option, mapping None to Color::None.
    pub fn from_option(option: Option<Self>) -> Self {
        option.unwrap_or_default()
    }

    /// The color's associated team.
    pub fn team(self) -> Team {
        match self {
            Color::Red | Color::Blue => Team::One,
            Color::Yellow | Color::Green => Team::Two,
            Color::None => Team::None
        }
    }

    /// Converts the color into an Option, mapping Color::None to None.
    pub fn to_option(self) -> Option<Self> {
        match self {
            Self::None => None,
            c => Some(c)
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::None
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

/// Serialized representation of a color.
#[derive(Serialize, Deserialize)]
struct PackedColor {
    #[serde(rename = "$value")]
    raw: String
}

impl From<Color> for PackedColor {
    fn from(color: Color) -> Self {
        Self { raw: color.to_string() }
    }
}

impl TryFrom<PackedColor> for Color {
    type Error = SCError;

    fn try_from(packed: PackedColor) -> Result<Self, SCError> {
        Self::from_str(packed.raw)
    }
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        PackedColor::from(self).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        PackedColor::deserialize(deserializer)
            .and_then(|p| Self::try_from(p).map_err(|e| D::Error::custom(format!("Could not deserialize color: {:?}", e))))
    }
}
