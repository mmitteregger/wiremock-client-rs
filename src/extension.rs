use std::collections::HashMap;
use std::iter::FromIterator;

use serde::Serialize;
use crate::common::Metadata;

pub struct Parameters {
    parameters: serde_json::Map<String, serde_json::Value>,
}

impl Parameters {
    pub fn empty() -> Parameters {
        Parameters {
            parameters: serde_json::Map::new(),
        }
    }

    pub fn one<K, V>(name: K, value: V) -> Parameters
        where K: Into<String>, V: Into<serde_json::Value>
    {
        let mut parameters = serde_json::Map::with_capacity(1);
        parameters.insert(name.into(), value.into());

        Parameters {
            parameters,
        }
    }

    pub fn of<T>(data: T) -> Parameters
        where T: Serialize
    {
        let value = serde_json::to_value(data).unwrap();

        if let serde_json::Value::Object(parameters) = value {
            Parameters {
                parameters,
            }
        } else {
            panic!("data argument was not converted to json object, but is: {}", value)
        }
    }
}

impl From<serde_json::Map<String, serde_json::Value>> for Parameters {
    fn from(metadata: serde_json::Map<String, serde_json::Value>) -> Parameters {
        Parameters::from_iter(metadata.into_iter())
    }
}

impl From<HashMap<String, serde_json::Value>> for Parameters {
    fn from(parameters: HashMap<String, serde_json::Value>) -> Parameters {
        Parameters::from_iter(parameters.into_iter())
    }
}

impl FromIterator<(String, serde_json::Value)> for Parameters {
    fn from_iter<T: IntoIterator<Item=(String, serde_json::Value)>>(iter: T) -> Parameters {
        Parameters {
            parameters: serde_json::Map::from_iter(iter),
        }
    }
}

impl Into<Metadata> for Parameters {
    fn into(self) -> Metadata {
        Metadata {
            metadata: self.parameters,
        }
    }
}
