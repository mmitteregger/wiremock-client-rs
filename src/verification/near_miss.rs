use serde::{Deserialize, Serialize};

use crate::verification::LoggedRequest;
use crate::stubbing::StubMapping;
use crate::matching::{RequestPattern, MatchResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct NearMiss {
    request: LoggedRequest,
    #[serde(rename = "stubMapping")]
    stub_mapping: StubMapping,
    #[serde(rename = "requestPattern")]
    request_pattern: Option<RequestPattern>,
    #[serde(rename = "matchResult")]
    match_result: MatchResult,
}

impl NearMiss {
    pub fn request(&self) -> &LoggedRequest {
        &self.request
    }
    pub fn stub_mapping(&self) -> &StubMapping {
        &self.stub_mapping
    }
    pub fn request_pattern(&self) -> Option<&RequestPattern> {
        self.request_pattern.as_ref()
    }
    pub fn match_result(&self) -> MatchResult {
        self.match_result
    }
}
