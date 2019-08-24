use serde::{Deserialize, Serialize};

use crate::extension::Parameters;
use crate::http::DelayDistribution;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GlobalSettings {
    #[serde(rename = "fixedDelay")]
    fixed_delay: Option<u16>,
    #[serde(rename = "delayDistribution")]
    delay_distribution: Option<DelayDistribution>,
    #[serde(default, skip_serializing_if = "Parameters::is_empty")]
    extended: Parameters,
}

impl GlobalSettings {
    pub fn fixed_delay(&self) -> Option<u16> {
        self.fixed_delay
    }

    pub fn delay_distribution(&self) -> Option<&DelayDistribution> {
        self.delay_distribution.as_ref()
    }

    pub fn extended(&self) -> &Parameters {
        &self.extended
    }

    pub fn clone_to_builder(&self) -> GlobalSettingsBuilder {
        GlobalSettingsBuilder {
            fixed_delay: self.fixed_delay,
            delay_distribution: self.delay_distribution.clone(),
            extended: self.extended.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GlobalSettingsBuilder {
    fixed_delay: Option<u16>,
    delay_distribution: Option<DelayDistribution>,
    extended: Parameters,
}

impl GlobalSettingsBuilder {
    pub fn new() -> GlobalSettingsBuilder {
        GlobalSettingsBuilder {
            fixed_delay: None,
            delay_distribution: None,
            extended: Parameters::empty(),
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

    pub fn extended<P>(mut self, extended: P) -> GlobalSettingsBuilder
        where P: Into<Parameters>
    {
        self.extended = extended.into();
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
