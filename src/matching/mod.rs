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
    /// The HTTP request method e.g. GET
    pub(crate) method: RequestMethod,
    /// The url pattern to match exactly against.
    #[serde(flatten)]
    pub(crate) url_pattern: UrlPattern,
    /// Query parameter patterns to match against.
    #[serde(rename = "queryParameters", default, skip_serializing_if = "IndexMap::is_empty")]
    pub(crate) query_params: IndexMap<String, ContentPattern>,
    /// Header patterns to match against.
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub(crate) headers: IndexMap<String, ContentPattern>,
    /// Cookie patterns to match against.
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub(crate) cookies: IndexMap<String, ContentPattern>,
    /// Pre-emptive basic auth credentials to match against.
    #[serde(rename = "basicAuthCredentials", skip_serializing_if = "Option::is_none")]
    pub(crate) basic_auth_credentials: Option<BasicCredentials>,
    /// Request body patterns to match against.
    #[serde(rename = "bodyPatterns", default, skip_serializing_if = "Vec::is_empty")]
    pub(crate) body_patterns: Vec<ContentPattern>,
}

impl RequestPattern {
    pub fn url_pattern(&self) -> &UrlPattern {
        &self.url_pattern
    }

    pub fn method(&self) -> &RequestMethod {
        &self.method
    }

    pub fn query_params(&self) -> &IndexMap<String, ContentPattern> {
        &self.query_params
    }

    pub fn query_params_mut(&mut self) -> &mut IndexMap<String, ContentPattern> {
        &mut self.query_params
    }

    pub fn headers(&self) -> &IndexMap<String, ContentPattern> {
        &self.headers
    }

    pub fn headers_mut(&mut self) -> &mut IndexMap<String, ContentPattern> {
        &mut self.headers
    }

    pub fn cookies(&self) -> &IndexMap<String, ContentPattern> {
        &self.cookies
    }

    pub fn cookies_mut(&mut self) -> &mut IndexMap<String, ContentPattern> {
        &mut self.cookies
    }

    pub fn basic_auth_credentials(&self) -> Option<&BasicCredentials> {
        self.basic_auth_credentials.as_ref()
    }

    pub fn body_patterns(&self) -> &[ContentPattern] {
        &self.body_patterns
    }

    pub fn body_patterns_mut(&mut self) -> &mut Vec<ContentPattern> {
        &mut self.body_patterns
    }
}

impl From<RequestPatternBuilder> for RequestPattern {
    fn from(builder: RequestPatternBuilder) -> RequestPattern {
        builder.build()
    }
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
    #[doc(hidden)]
    __Nonexhaustive,
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
