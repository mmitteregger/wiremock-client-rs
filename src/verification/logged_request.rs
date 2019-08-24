#[cfg(feature = "decode")]
use std::borrow::Cow;
use serde::{Deserialize, Serialize};
use indexmap::IndexMap;
use http::{HeaderMap, HeaderValue};
use http::header::AsHeaderName;
#[cfg(feature = "decode")]
use encoding_rs::{Encoding, UTF_8};
#[cfg(feature = "decode")]
use mime::Mime;

use crate::http::{RequestMethod, Cookie, QueryParameter};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggedRequest {
    url: String,
    #[serde(rename = "absoluteUrl")]
    absolute_url: String,
    method: RequestMethod,
    #[serde(rename = "clientIp")]
    client_ip: String,
    #[serde(default, skip_serializing_if = "HeaderMap::is_empty", with = "crate::serde::header_map")]
    headers: HeaderMap,
    #[serde(default)]
    cookies: IndexMap<String, Cookie>,
    #[serde(rename = "browserProxyRequest")]
    browser_proxy_request: bool,
    /// Timestamp epoch millis.
    #[serde(rename = "loggedDate")]
    logged_date: i64,
    #[serde(rename = "bodyAsBase64", default, with = "crate::serde::base64")]
    body: Vec<u8>,
    scheme: String,
    host: String,
    port: u16,
    #[serde(rename = "loggedDateString")]
    logged_date_string: String,
    #[serde(rename = "queryParams", default)]
    query_params: IndexMap<String, QueryParameter>,
}

impl LoggedRequest {
    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn absolute_url(&self) -> &str {
        &self.absolute_url
    }

    pub fn method(&self) -> &RequestMethod {
        &self.method
    }

    pub fn client_ip(&self) -> &str {
        &self.client_ip
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

    pub fn cookies(&self) -> &IndexMap<String, Cookie> {
        &self.cookies
    }

    pub fn is_browser_proxy_request(&self) -> bool {
        self.browser_proxy_request
    }

    pub fn logged_date(&self) -> i64 {
        self.logged_date
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

    pub fn scheme(&self) -> &str {
        &self.scheme
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn logged_date_string(&self) -> &str {
        &self.logged_date_string
    }

    pub fn query_params(&self) -> &IndexMap<String, QueryParameter> {
        &self.query_params
    }
}
