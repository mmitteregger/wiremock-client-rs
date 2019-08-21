use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DelayDistribution {
    #[serde(rename = "lognormal")]
    LogNormal {
        median: f64,
        sigma: f64,
    },
    #[serde(rename = "uniform")]
    Uniform {
        lower: u32,
        upper: u32,
    },
    #[doc(hidden)]
    __Nonexhaustive,
}
