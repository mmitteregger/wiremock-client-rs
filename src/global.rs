use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::http::DelayDistribution;

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalSettings {
    #[serde(rename = "fixedDelay")]
    fixed_delay: Option<u16>,
    #[serde(rename = "delayDistribution")]
    delay_distribution: Option<DelayDistribution>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    extended: HashMap<String, String>,
}

pub struct GlobalSettingsBuilder {
    fixed_delay: Option<u16>,
    delay_distribution: Option<DelayDistribution>,
    extended: HashMap<String, String>,
}

impl GlobalSettingsBuilder {
    pub fn new() -> GlobalSettingsBuilder {
        GlobalSettingsBuilder {
            fixed_delay: None,
            delay_distribution: None,
            extended: HashMap::new(),
        }
    }

    pub fn fixed_delay(mut self, fixed_delay: Option<u16>) -> GlobalSettingsBuilder {
        self.fixed_delay = fixed_delay;
        self
    }

    pub fn delay_distribution(mut self, delay_distribution: Option<DelayDistribution>) -> GlobalSettingsBuilder {
        self.delay_distribution = delay_distribution;
        self
    }

    pub fn extended(mut self, extended: HashMap<String, String>) -> GlobalSettingsBuilder {
        self.extended = extended;
        self
    }

    pub fn build(self) -> GlobalSettings {
        GlobalSettings {
            fixed_delay: self.fixed_delay,
            delay_distribution: self.delay_distribution,
            extended: self.extended,
        }
    }
}
