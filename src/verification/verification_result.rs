use serde::{Deserialize, Serialize};

use crate::verification::journal_based_result::{self, JournalBasedResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationResult {
    #[serde(with = "crate::serde::u32_negative_to_option")]
    count: Option<u32>,
    #[serde(rename = "requestJournalDisabled")]
    request_journal_disabled: bool,
}

impl VerificationResult {
    pub fn count(&self) -> Option<u32> {
        self.count
    }
}

impl JournalBasedResult for VerificationResult {
    fn request_journal_disabled(&self) -> bool {
        self.request_journal_disabled
    }

    fn assert_request_journal_enabled(&self) {
        journal_based_result::assert_request_journal_enabled(self.request_journal_disabled);
    }
}
