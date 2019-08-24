use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::iter::FromIterator;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde::export::fmt::Display;

use crate::extension::Parameters;

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

    pub fn from<T>(data: T) -> Metadata
        where T: Serialize,
    {
        match serde_json::to_value(data) {
            Ok(value) => value.into(),
            Err(error) => panic!("data cannot be converted to json value: {}", error),
        }
    }

    pub fn len(&self) -> usize {
        self.metadata.len()
    }

    pub fn is_empty(&self) -> bool {
        self.metadata.is_empty()
    }

    pub fn get<K: ?Sized>(&self, key: &K) -> Option<&serde_json::Value>
        where String: Borrow<K>,
              K: Ord + Eq + Hash + Display,
    {
        self.metadata.get(key)
    }

    pub fn get_str<K: ?Sized>(&self, key: &K) -> Option<&str>
        where String: Borrow<K>,
              K: Ord + Eq + Hash + Display,
    {
        self.metadata.get(key)
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
        self.metadata.get(key)
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
        self.metadata.get(key)
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
        self.metadata.get(key)
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
        self.metadata.get(key)
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
        self.metadata.get(key)
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
        self.metadata.get(key)
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
        self.metadata.get(key)
            .map(|value| {
                value.as_object()
                    .unwrap_or_else(|| {
                        panic!("key {} is not of type object, got: {}", key, value)
                    })
            })
    }

    pub fn get_metadata<K: ?Sized>(&self, key: &K) -> Option<Metadata>
        where String: Borrow<K>,
              K: Ord + Eq + Hash + Display,
    {
        self.metadata.get(key)
            .map(|value| {
                let object = value.as_object()
                    .unwrap_or_else(|| {
                        panic!("key {} is not of type object, got: {}", key, value)
                    });
                Metadata::from(object)
            })
    }
}

impl Default for Metadata {
    fn default() -> Metadata {
        Metadata::new()
    }
}

impl From<MetadataBuilder> for Metadata {
    fn from(builder: MetadataBuilder) -> Metadata {
        builder.build()
    }
}

impl From<&serde_json::Map<String, serde_json::Value>> for Metadata {
    fn from(metadata: &serde_json::Map<String, serde_json::Value>) -> Metadata {
        Metadata::from_iter(metadata.iter())
    }
}

impl From<serde_json::Map<String, serde_json::Value>> for Metadata {
    fn from(metadata: serde_json::Map<String, serde_json::Value>) -> Metadata {
        Metadata::from_iter(metadata.into_iter())
    }
}

impl From<serde_json::Value> for Metadata {
    fn from(value: serde_json::Value) -> Metadata {
        if let serde_json::Value::Object(metadata) = value {
            Metadata {
                metadata,
            }
        } else {
            panic!("argument needs not be converted to a json object, but is: {}", value)
        }
    }
}

impl From<HashMap<String, serde_json::Value>> for Metadata {
    fn from(metadata: HashMap<String, serde_json::Value>) -> Metadata {
        Metadata::from_iter(metadata.into_iter())
    }
}

impl From<IndexMap<String, serde_json::Value>> for Metadata {
    fn from(metadata: IndexMap<String, serde_json::Value>) -> Metadata {
        Metadata::from_iter(metadata.into_iter())
    }
}

impl From<Parameters> for Metadata {
    fn from(parameters: Parameters) -> Metadata {
        Metadata {
            metadata: parameters.parameters,
        }
    }
}

impl IntoIterator for Metadata {
    type Item = (String, serde_json::Value);
    type IntoIter = serde_json::map::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.metadata.into_iter()
    }
}

impl FromIterator<(String, serde_json::Value)> for Metadata {
    fn from_iter<T: IntoIterator<Item=(String, serde_json::Value)>>(iter: T) -> Metadata {
        Metadata {
            metadata: serde_json::Map::from_iter(iter),
        }
    }
}

impl<'a> FromIterator<(&'a String, &'a serde_json::Value)> for Metadata {
    fn from_iter<T: IntoIterator<Item=(&'a String, &'a serde_json::Value)>>(iter: T) -> Metadata {
        Metadata {
            metadata: serde_json::Map::from_iter(iter.into_iter().map(|(key, value)| (key.clone(), value.clone()))),
        }
    }
}

impl Into<serde_json::Value> for Metadata {
    fn into(self) -> serde_json::Value {
        serde_json::to_value(self.metadata).unwrap()
    }
}

pub fn metadata() -> MetadataBuilder {
    MetadataBuilder {
        metadata: serde_json::Map::new(),
    }
}

pub struct MetadataBuilder {
    metadata: serde_json::Map<String, serde_json::Value>,
}

impl MetadataBuilder {
    pub fn attr<K, V>(mut self, key: K, value: V) -> MetadataBuilder
        where K: Into<String>,
              V: Into<serde_json::Value>,
    {
        self.metadata.insert(key.into(), value.into());
        self
    }

    pub fn list<K, V, I>(mut self, key: K, values: V) -> MetadataBuilder
        where K: Into<String>,
              V: IntoIterator<Item=I>,
              I: Into<serde_json::Value>,
    {
        let value = values.into_iter()
            .map(|value| value.into())
            .collect::<Vec<serde_json::Value>>();

        self.metadata.insert(key.into(), serde_json::Value::Array(value));
        self
    }

    pub fn build(self) -> Metadata {
        Metadata {
            metadata: self.metadata,
        }
    }
}

impl From<serde_json::Map<String, serde_json::Value>> for MetadataBuilder {
    fn from(metadata: serde_json::Map<String, serde_json::Value>) -> MetadataBuilder {
        MetadataBuilder::from_iter(metadata.into_iter())
    }
}

impl From<serde_json::Value> for MetadataBuilder {
    fn from(value: serde_json::Value) -> MetadataBuilder {
        if let serde_json::Value::Object(metadata) = value {
            MetadataBuilder {
                metadata,
            }
        } else {
            panic!("argument needs not be converted to a json object, but is: {}", value)
        }
    }
}

impl From<HashMap<String, serde_json::Value>> for MetadataBuilder {
    fn from(metadata: HashMap<String, serde_json::Value>) -> MetadataBuilder {
        MetadataBuilder::from_iter(metadata.into_iter())
    }
}

impl From<Parameters> for MetadataBuilder {
    fn from(parameters: Parameters) -> MetadataBuilder {
        MetadataBuilder {
            metadata: parameters.parameters,
        }
    }
}

impl FromIterator<(String, serde_json::Value)> for MetadataBuilder {
    fn from_iter<T: IntoIterator<Item=(String, serde_json::Value)>>(iter: T) -> MetadataBuilder {
        MetadataBuilder {
            metadata: serde_json::Map::from_iter(iter),
        }
    }
}

impl Into<serde_json::Value> for MetadataBuilder {
    fn into(self) -> serde_json::Value {
        serde_json::to_value(self.metadata).unwrap()
    }
}
