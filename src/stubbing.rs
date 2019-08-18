use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::http::ResponseDefinition;
use crate::matching::RequestPattern;

#[derive(Debug, Serialize, Deserialize)]
pub struct StubMapping {
    /// This stub mapping's unique identifier.
    pub id: Uuid,
    /// The stub mapping's name.
    pub name: String,
    /// Request pattern that should be matched.
    pub request: RequestPattern,
    /// Response that should be returned when matched.
    pub response: ResponseDefinition,
    /// Indicates that the stub mapping should be persisted immediately
    /// on create/update/delete and survive resets to default.
    #[serde(default)]
    pub persistent: bool,
    /// This stub mapping's priority relative to others. 1 is highest.
    pub priority: u16,
    /// The name of the scenario that this stub mapping is part of.
    #[serde(rename = "scenarioName")]
    pub scenario_name: Option<String>,
    /// The required state of the scenario in order for this stub to be matched.
    #[serde(rename = "requiredScenarioState")]
    pub required_scenario_state: Option<String>,
    /// The new state for the scenario to be updated to after this stub is served.
    #[serde(rename = "newScenarioState")]
    pub new_scenario_state: Option<String>,
    /// A map of the names of post serve action extensions to trigger and their parameters.
    #[serde(rename = "postServeActions")]
    pub post_serve_actions: serde_json::Map<String, serde_json::Value>,
    /// Arbitrary metadata to be used for e.g. tagging, documentation.
    /// Can also be used to find and remove stubs.
    pub metadata: serde_json::Map<String, serde_json::Value>,
}
