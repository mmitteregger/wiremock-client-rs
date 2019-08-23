use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::collections::HashSet;
use crate::stubbing::StubMapping;

#[derive(Debug, Serialize, Deserialize)]
pub struct Scenario {
    id: Uuid,
    name: String,
    state: String,
    #[serde(rename = "possibleStates")]
    possible_states: HashSet<String>,
    mappings: Vec<StubMapping>,
}

impl Scenario {
    pub const STARTED: &'static str = "Started";

    pub fn in_started_state<S: Into<String>>(name: S) -> Scenario {
        Scenario {
            id: Uuid::new_v4(),
            name: name.into(),
            state: Scenario::STARTED.to_string(),
            possible_states: HashSet::new(),
            mappings: Vec::new(),
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn state(&self) -> &str {
        &self.state
    }

    pub fn possible_states(&self) -> &HashSet<String> {
        &self.possible_states
    }

    pub fn mappings(&self) -> &[StubMapping] {
        &self.mappings
    }
}
