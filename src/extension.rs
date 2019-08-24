use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::iter::FromIterator;

use serde::{Deserialize, Serialize};
use serde::export::fmt::Display;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Parameters {
    #[serde(flatten)]
    pub(crate) parameters: serde_json::Map<String, serde_json::Value>,
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
            panic!("argument needs not be converted to a json object, but is: {}", value)
        }
    }

    pub fn is_empty(&self) -> bool {
        self.parameters.is_empty()
    }

    pub fn get<K: ?Sized>(&self, key: &K) -> Option<&serde_json::Value>
        where String: Borrow<K>,
              K: Ord + Eq + Hash + Display,
    {
        self.parameters.get(key)
    }

    pub fn get_str<K: ?Sized>(&self, key: &K) -> Option<&str>
        where String: Borrow<K>,
              K: Ord + Eq + Hash + Display,
    {
        self.parameters.get(key)
            .map(|value| {
                value.as_str()
                    .unwrap_or_else(|| {
                        panic!("key {} is not of type String, got: {}", key, value)
                    })
            })
    }

    pub fn get_i64<K: ?Sized>(&self, key: &K) -> Option<i64>
        where String: Borrow<K>,
              K: Ord + Eq + Hash + Display,
    {
        self.parameters.get(key)
            .map(|value| {
                value.as_i64()
                    .unwrap_or_else(|| {
                        panic!("key {} is not of type i64, got: {}", key, value)
                    })
            })
    }

    pub fn get_u64<K: ?Sized>(&self, key: &K) -> Option<u64>
        where String: Borrow<K>,
              K: Ord + Eq + Hash + Display,
    {
        self.parameters.get(key)
            .map(|value| {
                value.as_u64()
                    .unwrap_or_else(|| {
                        panic!("key {} is not of type u64, got: {}", key, value)
                    })
            })
    }

    pub fn get_f64<K: ?Sized>(&self, key: &K) -> Option<f64>
        where String: Borrow<K>,
              K: Ord + Eq + Hash + Display,
    {
        self.parameters.get(key)
            .map(|value| {
                value.as_f64()
                    .unwrap_or_else(|| {
                        panic!("key {} is not of type f64, got: {}", key, value)
                    })
            })
    }

    pub fn get_bool<K: ?Sized>(&self, key: &K) -> Option<bool>
        where String: Borrow<K>,
              K: Ord + Eq + Hash + Display,
    {
        self.parameters.get(key)
            .map(|value| {
                value.as_bool()
                    .unwrap_or_else(|| {
                        panic!("key {} is not of type bool, got: {}", key, value)
                    })
            })
    }

    pub fn get_array<K: ?Sized>(&self, key: &K) -> Option<&Vec<serde_json::Value>>
        where String: Borrow<K>,
              K: Ord + Eq + Hash + Display,
    {
        self.parameters.get(key)
            .map(|value| {
                value.as_array()
                    .unwrap_or_else(|| {
                        panic!("key {} is not of type array, got: {}", key, value)
                    })
            })
    }

    pub fn get_mapped_array<K: ?Sized, F, V>(&self, key: &K, value_mapping: F) -> Option<Vec<V>>
        where String: Borrow<K>,
              K: Ord + Eq + Hash + Display,
              F: Fn(&serde_json::Value) -> V,
    {
        self.parameters.get(key)
            .map(|value| {
                value.as_array()
                    .unwrap_or_else(|| {
                        panic!("key {} is not of type array, got: {}", key, value)
                    })
                    .into_iter()
                    .map(|value| value_mapping(value))
                    .collect::<Vec<V>>()
            })
    }

    pub fn get_object<K: ?Sized>(&self, key: &K) -> Option<&serde_json::Map<String, serde_json::Value>>
        where String: Borrow<K>,
              K: Ord + Eq + Hash + Display,
    {
        self.parameters.get(key)
            .map(|value| {
                value.as_object()
                    .unwrap_or_else(|| {
                        panic!("key {} is not of type object, got: {}", key, value)
                    })
            })
    }

    pub fn get_parameters<K: ?Sized>(&self, key: &K) -> Option<Parameters>
        where String: Borrow<K>,
              K: Ord + Eq + Hash + Display,
    {
        self.parameters.get(key)
            .map(|value| {
                let object = value.as_object()
                    .unwrap_or_else(|| {
                        panic!("key {} is not of type object, got: {}", key, value)
                    });
                object.into()
            })
    }
}

impl From<&serde_json::Map<String, serde_json::Value>> for Parameters {
    fn from(parameters: &serde_json::Map<String, serde_json::Value>) -> Parameters {
        Parameters::from_iter(parameters.iter())
    }
}

impl From<serde_json::Map<String, serde_json::Value>> for Parameters {
    fn from(parameters: serde_json::Map<String, serde_json::Value>) -> Parameters {
        Parameters::from_iter(parameters.into_iter())
    }
}

impl From<serde_json::Value> for Parameters {
    fn from(value: serde_json::Value) -> Parameters {
        if let serde_json::Value::Object(parameters) = value {
            Parameters {
                parameters,
            }
        } else {
            panic!("argument needs not be converted to a json object, but is: {}", value)
        }
    }
}

impl From<HashMap<String, serde_json::Value>> for Parameters {
    fn from(parameters: HashMap<String, serde_json::Value>) -> Parameters {
        Parameters::from_iter(parameters.into_iter())
    }
}

impl IntoIterator for Parameters {
    type Item = (String, serde_json::Value);
    type IntoIter = serde_json::map::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.parameters.into_iter()
    }
}

impl FromIterator<(String, serde_json::Value)> for Parameters {
    fn from_iter<T: IntoIterator<Item=(String, serde_json::Value)>>(iter: T) -> Parameters {
        Parameters {
            parameters: serde_json::Map::from_iter(iter),
        }
    }
}

impl<'a> FromIterator<(&'a String, &'a serde_json::Value)> for Parameters {
    fn from_iter<T: IntoIterator<Item=(&'a String, &'a serde_json::Value)>>(iter: T) -> Parameters {
        Parameters {
            parameters: serde_json::Map::from_iter(iter.into_iter().map(|(key, value)| (key.clone(), value.clone()))),
        }
    }
}

impl Into<serde_json::Value> for Parameters {
    fn into(self) -> serde_json::Value {
        serde_json::to_value(self.parameters).unwrap()
    }
}
