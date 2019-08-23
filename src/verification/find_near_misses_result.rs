use serde::{Deserialize, Serialize};

use crate::verification::NearMiss;

#[derive(Debug, Serialize, Deserialize)]
pub struct FindNearMissesResult {
    #[serde(rename = "nearMisses", default)]
    near_misses: Vec<NearMiss>,
}

impl FindNearMissesResult {
    pub fn near_misses(&self) -> &[NearMiss] {
        &self.near_misses
    }
}

impl Into<Vec<NearMiss>> for FindNearMissesResult {
    fn into(self) -> Vec<NearMiss> {
        self.near_misses
    }
}
