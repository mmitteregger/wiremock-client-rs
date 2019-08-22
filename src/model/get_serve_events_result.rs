use serde::{Serialize, Deserialize};

use crate::model::pagination::{Meta, PaginatedResult};
use crate::stubbing::ServeEvent;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetServeEventsResult {
    requests: Vec<ServeEvent>,
    meta: Meta,
    #[serde(rename = "requestJournalDisabled")]
    request_journal_disabled: bool,
}

impl PaginatedResult<ServeEvent> for GetServeEventsResult {
    fn selection(&self) -> &[ServeEvent] {
        &self.requests
    }

    fn selection_mut(&mut self) -> &mut Vec<ServeEvent> {
        &mut self.requests
    }

    fn meta(&self) -> &Meta {
        &self.meta
    }
}

impl GetServeEventsResult {
    pub fn serve_events(&self) -> &[ServeEvent] {
        &self.requests
    }

    pub fn serve_events_mut(&mut self) -> &mut Vec<ServeEvent> {
        &mut self.requests
    }
}

impl Into<Vec<ServeEvent>> for GetServeEventsResult {
    fn into(self) -> Vec<ServeEvent> {
        self.requests
    }
}
