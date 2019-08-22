use serde::{Serialize, Deserialize};

use crate::stubbing::ServeEvent;
use crate::model::SingleItemResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct SingleServedStubResult {
    #[serde(flatten)]
    serve_event: ServeEvent,
}

impl SingleItemResult<ServeEvent> for SingleServedStubResult {
    fn item(&self) -> &ServeEvent {
        &self.serve_event
    }

    fn item_mut(&mut self) -> &mut ServeEvent {
        &mut self.serve_event
    }
}

impl Into<ServeEvent> for SingleServedStubResult {
    fn into(self) -> ServeEvent {
        self.serve_event
    }
}
