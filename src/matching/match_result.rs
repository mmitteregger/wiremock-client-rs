use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct MatchResult {
    distance: f64,
}

impl MatchResult {
    pub fn distance(&self) -> f64 {
        self.distance
    }

    pub fn is_exact_match(&self) -> bool {
        self.distance == 0f64
    }
}
