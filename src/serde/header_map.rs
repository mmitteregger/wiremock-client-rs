use std::cell::RefCell;
use std::fmt;
use std::ops::DerefMut;
use std::str::FromStr;

use http::{HeaderMap, HeaderValue};
use http::header::{Entry, HeaderName, ValueIter};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{self, MapAccess, Visitor};
use serde::ser::{self, SerializeMap, SerializeSeq};

struct MultipleHeaderValues<'a> {
    name: &'a HeaderName,
    first: &'a HeaderValue,
    second: &'a HeaderValue,
    others: RefCell<ValueIter<'a, HeaderValue>>,
}

impl<'a> Serialize for MultipleHeaderValues<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut serialize_seq = serializer.serialize_seq(None)?;

        let first_value_str = value_to_str(self.name, self.first)?;
        serialize_seq.serialize_element(first_value_str)?;

        let second_value_str = value_to_str(self.name, self.second)?;
        serialize_seq.serialize_element(second_value_str)?;

        let mut others_ref = self.others.borrow_mut();
        let others = others_ref.deref_mut();

        for other_value in others {
            let other_value_str = value_to_str(self.name, other_value)?;
            serialize_seq.serialize_element(other_value_str)?;
        }

        serialize_seq.end()
    }
}

pub fn serialize<S>(header_map: &HeaderMap, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
{
    let mut serialize_map = serializer.serialize_map(Some(header_map.keys_len()))?;

    for name in header_map.keys() {
        serialize_map.serialize_key(name.as_str())?;

        let values = header_map.get_all(name.as_str());
        let mut values_iter = values.iter();

        let opt_first_value = values_iter.next();
        let opt_second_value = values_iter.next();

        let first_value = match opt_first_value {
            Some(first_value) => first_value,
            None => {
                let msg = format!("header name \"{}\" does not contain at least \
                        a single header value", name);
                return Err(ser::Error::custom(msg))
            },
        };

        if let Some(second_value) = opt_second_value {
            let all_values = MultipleHeaderValues {
                name,
                first: first_value,
                second: second_value,
                others: RefCell::new(values_iter),
            };
            serialize_map.serialize_value(&all_values)?;
        } else {
            let first_value_str = value_to_str(name, first_value)?;
            serialize_map.serialize_value(first_value_str)?;
        }
    }

    serialize_map.end()
}

fn value_to_str<'v, E>(name: &HeaderName, value: &'v HeaderValue) -> Result<&'v str, E>
    where E: ser::Error + Sized
{
    value.to_str()
        .map_err(|_to_str_error| {
            let msg = format!("header value for header \"{}\" \
                                contains invalid UTF-8 characters", name);
            ser::Error::custom(msg)
        })
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<HeaderMap, D::Error>
    where D: Deserializer<'de>
{
    deserializer.deserialize_map(HeaderMapVisitor)
}

#[derive(Deserialize)]
#[serde(untagged)]
enum StrOrString<'a> {
    Str(&'a str),
    String(String),
}

impl<'a> AsRef<str> for StrOrString<'a> {
    fn as_ref(&self) -> &str {
        match self {
            StrOrString::Str(borrowed_string) => borrowed_string,
            StrOrString::String(owned_string) => owned_string.as_str(),
        }
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum StrStringOrSeq<'a> {
    Str(&'a str),
    OwnedStrSeq(Vec<&'a str>),
    String(String),
    OwnedStringSeq(Vec<String>),
}

struct HeaderMapVisitor;

impl<'de> Visitor<'de> for HeaderMapVisitor {
    type Value = HeaderMap;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        formatter.write_str("a map of headers with values as string or string array")
    }

    fn visit_map<M>(self, mut map_access: M) -> Result<HeaderMap, M::Error>
        where M: MapAccess<'de>,
    {
        let mut headers = HeaderMap::with_capacity(map_access.size_hint().unwrap_or(0));

        while let Some(str_or_string) = map_access.next_key::<StrOrString>()? {
            let key = str_or_string.as_ref();
            let header_name = match HeaderName::from_str(key) {
                Ok(name) => name,
                Err(_invalid_header_name_error) => {
                    return Err(de::Error::custom(format!("header name \"{}\" is invalid", key)));
                },
            };

            match map_access.next_value::<StrStringOrSeq>()? {
                StrStringOrSeq::Str(value) => {
                    let header_value = to_header_value(value)?;
                    headers.append(header_name, header_value);
                },
                StrStringOrSeq::OwnedStrSeq(values) => {
                    let values_iter = values.iter()
                        .map(|value| *value);
                    add_header_values(&mut headers, &header_name, values_iter)?;
                }
                StrStringOrSeq::String(value) => {
                    let header_value = to_header_value(&value)?;
                    headers.append(header_name, header_value);
                }
                StrStringOrSeq::OwnedStringSeq(values) => {
                    let values_iter = values.iter()
                        .map(|value| value.as_str());
                    add_header_values(&mut headers, &header_name, values_iter)?;
                }
            }
        }

        Ok(headers)
    }
}

fn to_header_value<E>(value: &str) -> Result<HeaderValue, E> where E: de::Error + Sized {
    HeaderValue::from_str(value)
        .map_err(|_invalid_header_value_error| {
            de::Error::custom(format!("header value \"{}\" is invalid", value))
        })
}

fn add_header_values<'a, I, E>(headers: &mut HeaderMap, header_name: &HeaderName, values: I)
    -> Result<(), E>
    where I: IntoIterator<Item=&'a str>,
          E: de::Error + Sized,
{
    let entry = headers.entry(header_name).unwrap();

    let mut values_iter = values.into_iter();
    let mut occupied_entry;

    match values_iter.next() {
        Some(first_value) => {
            let header_value = to_header_value(first_value)?;

            match entry {
                Entry::Occupied(mut entry) => {
                    entry.append(header_value);
                    occupied_entry = entry;
                },
                Entry::Vacant(entry) => {
                    occupied_entry = entry.insert_entry(header_value);
                },
            };
        },
        None => return Ok(()),
    }

    for value in values_iter {
        let header_value = to_header_value(value)?;
        occupied_entry.append(header_value);
    }

    Ok(())
}
