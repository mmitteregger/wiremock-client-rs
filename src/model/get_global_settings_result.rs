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

    pub fn into_settings(self) -> GlobalSettings {
        self.settings
    }
}
