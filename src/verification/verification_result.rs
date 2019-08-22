use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationResult {
    #[serde(with = "crate::serde::u16_negative_to_option")]
    count: Option<u16>,
    #[serde(rename = "requestJournalDisabled")]
    request_journal_disabled: bool,
}

impl VerificationResult {
    pub fn count(&self) -> Option<u16> {
        self.count
    }
    pub fn request_journal_disabled(&self) -> bool {
        self.request_journal_disabled
    }
}
