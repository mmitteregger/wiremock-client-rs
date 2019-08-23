use serde::{Serialize, Deserialize};

use crate::stubbing::Scenario;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetScenariosResult {
    scenarios: Vec<Scenario>,
}

impl GetScenariosResult {
    pub fn scenarios(&self) -> &[Scenario] {
        &self.scenarios
    }
}

impl Into<Vec<Scenario>> for GetScenariosResult {
    fn into(self) -> Vec<Scenario> {
        self.scenarios
    }
}
