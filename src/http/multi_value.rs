use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum MultiValue {
    Single(String),
    Multi(Vec<String>),
}
