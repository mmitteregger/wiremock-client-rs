use serde::{Deserialize, Serialize};

use crate::verification::LoggedRequest;
use crate::verification::journal_based_result::{self, JournalBasedResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct FindRequestsResult {
    #[serde(default)]
    requests: Vec<LoggedRequest>,
    #[serde(rename = "requestJournalDisabled")]
    request_journal_disabled: bool,
}

impl FindRequestsResult {
    pub fn requests(&self) -> &[LoggedRequest] {
        &self.requests
    }
    pub fn requests_mut(&mut self) -> &mut Vec<LoggedRequest> {
        &mut self.requests
    }
}

impl JournalBasedResult for FindRequestsResult {
    fn request_journal_disabled(&self) -> bool {
        self.request_journal_disabled
    }

    fn assert_request_journal_enabled(&self) {
        journal_based_result::assert_request_journal_enabled(self.request_journal_disabled);
    }
}

impl Into<Vec<LoggedRequest>> for FindRequestsResult {
    fn into(self) -> Vec<LoggedRequest> {
        self.requests
    }
}
