#[cfg(feature = "decode")]
use std::borrow::Cow;
use serde::{Deserialize, Serialize};
use http::{HeaderMap, HeaderValue};
use http::header::AsHeaderName;
#[cfg(feature = "decode")]
use mime::Mime;
#[cfg(feature = "decode")]
use encoding_rs::{Encoding, UTF_8};

use crate::http::Fault;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggedResponse {
    status: u16,
    #[serde(default, skip_serializing_if = "HeaderMap::is_empty", with = "crate::serde::header_map")]
    headers: HeaderMap,
    #[serde(rename = "bodyAsBase64", default, with = "crate::serde::base64")]
    body: Vec<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fault: Option<Fault>,
}

impl LoggedResponse {
    pub fn status(&self) -> u16 {
        self.status
    }

    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    pub fn first_header<K: AsHeaderName>(&self, key: K) -> Option<&HeaderValue> {
        self.headers.get(key)
    }

    pub fn first_header_str<K: AsHeaderName>(&self, key: K) -> Option<&str> {
        self.headers.get(key)
            .and_then(|value| value.to_str().ok())
    }

    #[cfg(feature = "decode")]
    pub fn content_type_header(&self) -> Option<Mime> {
        self.first_header_str(http::header::CONTENT_TYPE)
            .and_then(|content_type| content_type.parse::<Mime>().ok())
    }

    pub fn body(&self) -> &[u8] {
        &self.body
    }

    #[cfg(feature = "decode")]
    pub fn decode_body(&self) -> Option<Cow<str>> {
        self.encoding_from_content_type_header()
            .unwrap_or(UTF_8)
            .decode_without_bom_handling_and_without_replacement(self.body())
    }

    #[cfg(feature = "decode")]
    pub fn encoding_from_content_type_header(&self) -> Option<&'static Encoding> {
        if let Some(content_type) = self.content_type_header() {
            if let Some(charset_name) = content_type.get_param("charset") {
                let charset = charset_name.as_str();
                return Encoding::for_label(charset.as_bytes());
            }
        }

        None
    }

    pub fn fault(&self) -> Option<Fault> {
        self.fault
    }
}
