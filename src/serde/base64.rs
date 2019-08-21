use std::fmt;

use serde::{Deserializer, Serializer};
use serde::de::{self, Unexpected, Visitor};

pub fn serialize<S>(value: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
{
    serializer.serialize_str(&base64::encode(value))
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where D: Deserializer<'de>
{
    deserializer.deserialize_str(Base64StrVisitor)
}

struct Base64StrVisitor;

impl<'de> Visitor<'de> for Base64StrVisitor {
    type Value = Vec<u8>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        formatter.write_str("a string representation of base64 encoded data")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where E: de::Error,
    {
        base64::decode(value)
            .map_err(|_decode_error| {
                E::invalid_value(Unexpected::Str(value), &self)
            })
    }
}
