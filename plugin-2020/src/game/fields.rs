use serde::{Serialize, Deserialize};
use super::Field;

/// The fields of the board. This is a separate
/// struct from Board due to the way it is represented
/// in XML.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Fields {
    #[serde(rename = "field")]
    fields: Vec<Field>
}

impl Fields {
    pub fn new() -> Self {
        Self { fields: Vec::new() }
    }

    pub fn iter(&self) -> impl Iterator<Item=Field> {
        self.fields.iter()
    }
}
