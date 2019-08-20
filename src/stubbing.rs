use serde::{Deserialize, Serialize};
use uuid::Uuid;
use indexmap::IndexMap;

use crate::http::ResponseDefinition;
use crate::matching::RequestPattern;
use crate::client::builder::{MappingBuilder, ScenarioMappingBuilder};

#[derive(Debug, Serialize, Deserialize)]
pub struct StubMapping {
    /// This stub mapping's unique identifier.
    pub id: Uuid,
    /// The stub mapping's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Request pattern that should be matched.
    pub request: RequestPattern,
    /// Response that should be returned when matched.
    pub response: ResponseDefinition,
    /// Indicates that the stub mapping should be persisted immediately
    /// on create/update/delete and survive resets to default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent: Option<bool>,
    /// This stub mapping's priority relative to others. 1 is highest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u16>,
    /// The name of the scenario that this stub mapping is part of.
    #[serde(rename = "scenarioName", skip_serializing_if = "Option::is_none")]
    pub scenario_name: Option<String>,
    /// The required state of the scenario in order for this stub to be matched.
    #[serde(rename = "requiredScenarioState", skip_serializing_if = "Option::is_none")]
    pub required_scenario_state: Option<String>,
    /// The new state for the scenario to be updated to after this stub is served.
    #[serde(rename = "newScenarioState", skip_serializing_if = "Option::is_none")]
    pub new_scenario_state: Option<String>,
    /// A map of the names of post serve action extensions to trigger and their parameters.
    #[serde(rename = "postServeActions", default, skip_serializing_if = "IndexMap::is_empty")]
    pub post_serve_actions: IndexMap<String, serde_json::Value>,
    /// Arbitrary metadata to be used for e.g. tagging, documentation.
    /// Can also be used to find and remove stubs.
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub metadata: IndexMap<String, serde_json::Value>,
}

impl From<MappingBuilder> for StubMapping {
    fn from(builder: MappingBuilder) -> StubMapping {
        builder.build()
    }
}

impl From<ScenarioMappingBuilder> for StubMapping {
    fn from(builder: ScenarioMappingBuilder) -> StubMapping {
        builder.build()
    }
}
