use serde::{Serialize, Deserialize};

use crate::global::GlobalSettings;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetGlobalSettingsResult {
    settings: GlobalSettings,
}

impl GetGlobalSettingsResult {
    pub fn settings(&self) -> &GlobalSettings {
        &self.settings
    }
}

impl Into<GlobalSettings> for GetGlobalSettingsResult {
    fn into(self) -> GlobalSettings {
        self.settings
    }
}
