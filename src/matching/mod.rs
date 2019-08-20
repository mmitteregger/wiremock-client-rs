use serde::{Deserialize, Serialize};
use indexmap::IndexMap;

use crate::client::BasicCredentials;
use crate::http::RequestMethod;
pub use crate::matching::content_pattern::*;
pub use crate::matching::builder::*;

mod content_pattern;
mod builder;

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestPattern {
    /// The url pattern to match exactly against.
    #[serde(flatten)]
    pub url: UrlPattern,
    /// The HTTP request method e.g. GET
    pub method: RequestMethod,
    /// Query parameter patterns to match against.
    #[serde(rename = "queryParameters", default, skip_serializing_if = "IndexMap::is_empty")]
    pub query_params: IndexMap<String, ContentPattern>,
    /// Header patterns to match against.
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub headers: IndexMap<String, ContentPattern>,
    /// Cookie patterns to match against.
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub cookies: IndexMap<String, ContentPattern>,
    /// Pre-emptive basic auth credentials to match against.
    #[serde(rename = "basicAuthCredentials", skip_serializing_if = "Option::is_none")]
    pub basic_auth_credentials: Option<BasicCredentials>,
    /// Request body patterns to match against.
    #[serde(rename = "bodyPatterns", default, skip_serializing_if = "Vec::is_empty")]
    pub body_patterns: Vec<ContentPattern>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UrlPattern {
    /// The path and query to match exactly against.
    #[serde(rename = "url")]
    Url(String),
    /// The path to match exactly against.
    #[serde(rename = "urlPath")]
    UrlPath(String),
    /// The path regex to match against.
    #[serde(rename = "urlPathPattern")]
    UrlPathPattern(String),
    /// The path and query regex to match against.
    #[serde(rename = "urlPattern")]
    UrlPattern(String),
}

impl UrlPattern {
    pub fn any() -> UrlPattern {
        UrlPattern::UrlPattern(".*".to_string())
    }
}

impl From<&str> for UrlPattern {
    fn from(url: &str) -> UrlPattern {
        UrlPattern::Url(url.to_owned())
    }
}

impl From<String> for UrlPattern {
    fn from(url: String) -> UrlPattern {
        UrlPattern::Url(url)
    }
}

//#[derive(Debug, Serialize, Deserialize)]
//pub struct MultipartValuePattern;
//
//#[derive(Debug, Serialize, Deserialize)]
//pub struct MultiValuePattern {
//    value_pattern: Box<StringValuePattern>,
//}
