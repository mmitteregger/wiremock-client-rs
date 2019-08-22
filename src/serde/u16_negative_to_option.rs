use std::fmt;
use std::convert::TryFrom;

use serde::{Deserializer, Serializer};
use serde::de::{self, Visitor};

pub fn serialize<S>(value: &Option<u16>, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
{
    match value {
        Some(value) => serializer.serialize_u16(*value),
        None => serializer.serialize_i8(-1),
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<u16>, D::Error>
    where D: Deserializer<'de>
{
    deserializer.deserialize_i32(U16NegativeToOptionVisitor)
}

struct U16NegativeToOptionVisitor;

impl<'de> Visitor<'de> for U16NegativeToOptionVisitor {
    type Value = Option<u16>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        formatter.write_str("a number that fits in a i32")
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> where E: de::Error {
        if value < 0 {
            Ok(None)
        } else {
            match u16::try_from(value) {
                Ok(value) => Ok(Some(value)),
                Err(_error) => Err(de::Error::custom(format!("number {} is too large for u16", value))),
            }
        }
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> where E: de::Error {
        match u16::try_from(value) {
            Ok(value) => Ok(Some(value)),
            Err(_error) => Err(de::Error::custom(format!("number {} is too large for u16", value))),
        }
    }
}
