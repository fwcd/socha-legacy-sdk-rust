use serde::{Serialize, Deserialize};

// TODO: Serialize may not serialize correctly to an attribute yet
// due to https://github.com/tafia/quick-xml/issues/283

/// Determines how scores should be aggregated (e.g. summed up or averaged over).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ScoreAggregation {
    Sum,
    Average
}
