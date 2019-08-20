use serde::{Serialize, Deserialize};

pub use pagination::{Meta, PageParams, PaginatedResult};

use crate::stubbing::StubMapping;
use crate::global::GlobalSettings;

mod pagination;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct GetGlobalSettingsResult {
    settings: GlobalSettings,
}

impl GetGlobalSettingsResult {
    pub fn settings(&self) -> &GlobalSettings {
        &self.settings
    }

    pub fn into_settings(self) -> GlobalSettings {
        self.settings
    }
}
