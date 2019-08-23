use serde::{Deserialize, Serialize};

use crate::verification::LoggedRequest;
use crate::stubbing::StubMapping;
use crate::matching::{RequestPattern, MatchResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct NearMiss {
    request: LoggedRequest,
    #[serde(rename = "stubMapping", skip_serializing_if = "Option::is_none")]
    stub_mapping: Option<StubMapping>,
    #[serde(rename = "requestPattern", skip_serializing_if = "Option::is_none")]
    request_pattern: Option<RequestPattern>,
    #[serde(rename = "matchResult")]
    match_result: MatchResult,
}

impl NearMiss {
    pub fn request(&self) -> &LoggedRequest {
        &self.request
    }
    pub fn stub_mapping(&self) -> Option<&StubMapping> {
        self.stub_mapping.as_ref()
    }
    pub fn request_pattern(&self) -> Option<&RequestPattern> {
        self.request_pattern.as_ref()
    }
    pub fn match_result(&self) -> MatchResult {
        self.match_result
    }
}
