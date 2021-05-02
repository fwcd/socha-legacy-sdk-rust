use serde::{Serialize, Deserialize};
use super::ScoreAggregation;

/// A single score fragment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScoreFragment {
    pub name: String,
    pub aggregation: ScoreAggregation,
    pub relevant_for_ranking: bool,
}
