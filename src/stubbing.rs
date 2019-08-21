use serde::{Deserialize, Serialize};
use uuid::Uuid;
use indexmap::IndexMap;

use crate::http::ResponseDefinition;
use crate::matching::RequestPattern;
use crate::client::builder::{MappingBuilder, ScenarioMappingBuilder};
use crate::common::Metadata;
use crate::extension::Parameters;

#[derive(Debug, Serialize, Deserialize)]
pub struct StubMapping {
    /// This stub mapping's unique identifier.
    pub(crate) id: Uuid,
    /// The stub mapping's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) name: Option<String>,
    /// Request pattern that should be matched.
    pub(crate) request: RequestPattern,
    /// Response that should be returned when matched.
    pub(crate) response: ResponseDefinition,
    /// Indicates that the stub mapping should be persisted immediately
    /// on create/update/delete and survive resets to default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) persistent: Option<bool>,
    /// This stub mapping's priority relative to others. 1 is highest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) priority: Option<u16>,
    /// The name of the scenario that this stub mapping is part of.
    #[serde(rename = "scenarioName", skip_serializing_if = "Option::is_none")]
    pub(crate) scenario_name: Option<String>,
    /// The required state of the scenario in order for this stub to be matched.
    #[serde(rename = "requiredScenarioState", skip_serializing_if = "Option::is_none")]
    pub(crate) required_scenario_state: Option<String>,
    /// The new state for the scenario to be updated to after this stub is served.
    #[serde(rename = "newScenarioState", skip_serializing_if = "Option::is_none")]
    pub(crate) new_scenario_state: Option<String>,
    /// A map of the names of post serve action extensions to trigger and their parameters.
    #[serde(rename = "postServeActions", default, skip_serializing_if = "IndexMap::is_empty")]
    pub(crate) post_serve_actions: IndexMap<String, Parameters>,
    /// Arbitrary metadata to be used for e.g. tagging, documentation.
    /// Can also be used to find and remove stubs.
    #[serde(default, skip_serializing_if = "Metadata::is_empty")]
    pub(crate) metadata: Metadata,
}

impl StubMapping {
	pub fn set_id(&mut self, id: Uuid) {
		self.id = id;
	}

	pub fn id(&self) -> &Uuid {
		&self.id
	}

    pub fn set_name<S>(&mut self, name: S) where S: Into<String> {
        self.name = Some(name.into());
    }

	pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|name| name.as_str())
	}

    pub fn set_persistent(&mut self, persistent: bool) {
        self.persistent = Some(persistent);
    }

    pub fn is_persistent(&self) -> bool {
        self.persistent.unwrap_or(false)
    }

    pub fn set_request<R>(&mut self, request: R) where R: Into<RequestPattern> {
        self.request = request.into();
    }

    pub fn request(&self) -> &RequestPattern {
		&self.request
	}

    pub fn set_response<R>(&mut self, response: R) where R: Into<ResponseDefinition> {
        self.response = response.into();
    }

	pub fn response(&self) -> &ResponseDefinition {
		&self.response
	}

	pub fn priority(&self) -> u16 {
		self.priority.unwrap_or(5)
	}

	pub fn set_priority(&mut self, priority: u16) {
		self.priority = Some(priority);
	}

	pub fn scenario_name(&self) -> Option<&str> {
		self.scenario_name.as_ref().map(|scenario_name| scenario_name.as_str())
	}

	pub fn set_scenario_name<S>(&mut self, scenario_name: S) where S: Into<String> {
		self.scenario_name = Some(scenario_name.into());
	}

	pub fn required_scenario_state(&self) -> Option<&str> {
        self.required_scenario_state.as_ref().map(|state| state.as_str())
	}

	pub fn set_required_scenario_state<S>(&mut self, required_scenario_state: S) where S: Into<String> {
		self.required_scenario_state = Some(required_scenario_state.into());
	}

	pub fn new_scenario_state(&self) -> Option<&str> {
        self.new_scenario_state.as_ref().map(|state| state.as_str())
	}

	pub fn set_new_scenario_state<S>(&mut self, new_scenario_state: S) where S: Into<String> {
		self.new_scenario_state = Some(new_scenario_state.into());
	}

	pub fn is_in_scenario(&self) -> bool {
		self.scenario_name.is_some()
	}

	pub fn modifies_scenario_state(&self) -> bool {
		self.new_scenario_state.is_some()
	}

    pub fn is_independent_of_scenario_state(&self) -> bool {
        !self.is_in_scenario() || self.required_scenario_state.is_none()
	}

    pub fn post_serve_actions(&self) -> &IndexMap<String, Parameters> {
        &self.post_serve_actions
    }

    pub fn post_serve_actions_mut(&mut self) -> &mut IndexMap<String, Parameters> {
        &mut self.post_serve_actions
    }

    pub fn set_post_serve_actions<A>(&mut self, post_serve_actions: A) where A: Into<IndexMap<String, Parameters>> {
        self.post_serve_actions = post_serve_actions.into();
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    pub fn metadata_mut(&mut self) -> &mut Metadata {
        &mut self.metadata
    }

    pub fn set_metadata<M>(&mut self, metadata: M) where M: Into<Metadata> {
        self.metadata = metadata.into();
    }
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

pub struct Scenario;

impl Scenario {
    pub const STARTED: &'static str = "Started";
}
