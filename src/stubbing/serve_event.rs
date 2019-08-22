use serde::{Deserialize, Serialize};
use uuid::Uuid;
use indexmap::IndexMap;

use crate::stubbing::StubMapping;
use crate::http::{ResponseDefinition, LoggedResponse};
use crate::extension::Parameters;
use crate::common::Timing;
use crate::verification::LoggedRequest;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServeEvent {
    id: Uuid,
    request: LoggedRequest,
    #[serde(rename = "responseDefinition")]
    response_definition: ResponseDefinition,
    response: LoggedResponse,
    #[serde(rename = "wasMatched")]
    was_matched: bool,
    timing: Timing,
    #[serde(rename = "stubMapping")]
    stub_mapping: StubMapping,
}

impl ServeEvent {
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn request(&self) -> &LoggedRequest {
        &self.request
    }

    pub fn request_mut(&mut self) -> &mut LoggedRequest {
        &mut self.request
    }

    pub fn response_definition(&self) -> &ResponseDefinition {
        &self.response_definition
    }

    pub fn response_definition_mut(&mut self) -> &mut ResponseDefinition {
        &mut self.response_definition
    }

    pub fn response(&self) -> &LoggedResponse {
        &self.response
    }

    pub fn response_mut(&mut self) -> &mut LoggedResponse {
        &mut self.response
    }

    pub fn was_matched(&self) -> bool {
        self.was_matched
    }

    pub fn timing(&self) -> Timing {
        self.timing
    }

    pub fn stub_mapping(&self) -> &StubMapping {
        &self.stub_mapping
    }

    pub fn stub_mapping_mut(&mut self) -> &mut StubMapping {
        &mut self.stub_mapping
    }

    pub fn post_serve_actions(&self) -> &IndexMap<String, Parameters> {
        &self.stub_mapping.post_serve_actions
    }

    pub fn post_serve_actions_mut(&mut self) -> &mut IndexMap<String, Parameters> {
        &mut self.stub_mapping.post_serve_actions
    }
}
