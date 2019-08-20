use std::collections::HashMap;
use std::iter::FromIterator;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(flatten)]
    pub(crate) metadata: serde_json::Map<String, serde_json::Value>,
}

impl Metadata {
    pub fn new() -> Metadata {
        Metadata {
            metadata: serde_json::Map::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Metadata {
        Metadata {
            metadata: serde_json::Map::with_capacity(capacity),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.metadata.is_empty()
    }
}

impl Default for Metadata {
    fn default() -> Metadata {
        Metadata::new()
    }
}

impl From<serde_json::Map<String, serde_json::Value>> for Metadata {
    fn from(metadata: serde_json::Map<String, serde_json::Value>) -> Metadata {
        Metadata::from_iter(metadata.into_iter())
    }
}

impl From<HashMap<String, serde_json::Value>> for Metadata {
    fn from(metadata: HashMap<String, serde_json::Value>) -> Metadata {
        Metadata::from_iter(metadata.into_iter())
    }
}

impl FromIterator<(String, serde_json::Value)> for Metadata {
    fn from_iter<T: IntoIterator<Item=(String, serde_json::Value)>>(iter: T) -> Metadata {
        Metadata {
            metadata: serde_json::Map::from_iter(iter),
        }
    }
}
