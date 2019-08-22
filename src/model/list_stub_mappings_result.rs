use serde::{Serialize, Deserialize};

use crate::model::pagination::{Meta, PaginatedResult};
use crate::stubbing::StubMapping;

#[derive(Debug, Serialize, Deserialize)]
pub struct ListStubMappingsResult {
    mappings: Vec<StubMapping>,
    meta: Meta,
}

impl PaginatedResult<StubMapping> for ListStubMappingsResult {
    fn selection(&self) -> &[StubMapping] {
        &self.mappings
    }

    fn meta(&self) -> &Meta {
        &self.meta
    }
}

impl ListStubMappingsResult {
    pub fn mappings(&self) -> &[StubMapping] {
        self.selection()
    }
}
