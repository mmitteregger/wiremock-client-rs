use serde::{Serialize, Deserialize};

use crate::stubbing::StubMapping;
use crate::model::SingleItemResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct SingleStubMappingResult {
    #[serde(flatten)]
    stub_mapping: StubMapping,
}

impl SingleItemResult<StubMapping> for SingleStubMappingResult {
    fn item(&self) -> &StubMapping {
        &self.stub_mapping
    }

    fn item_mut(&mut self) -> &mut StubMapping {
        &mut self.stub_mapping
    }
}

impl Into<StubMapping> for SingleStubMappingResult {
    fn into(self) -> StubMapping {
        self.stub_mapping
    }
}
