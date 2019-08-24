use std::borrow::Cow;
use std::fmt;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::any_url;
use crate::client::BasicCredentials;
use crate::http::RequestMethod;
use crate::matching::{ContentPattern, RequestPatternBuilder, UrlPattern};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestPattern {
    /// The url pattern to match exactly against.
    #[serde(flatten)]
    pub(crate) url_pattern: Option<UrlPattern>,
    /// The HTTP request method e.g. GET
    pub(crate) method: RequestMethod,
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
    pub fn everything() -> RequestPattern {
        RequestPatternBuilder::new(RequestMethod::ANY, any_url()).build()
    }

    pub fn url_pattern(&self) -> Option<&UrlPattern> {
        self.url_pattern.as_ref()
    }

    pub fn method(&self) -> &RequestMethod {
        &self.method
    }

    pub fn query_params(&self) -> &IndexMap<String, ContentPattern> {
        &self.query_params
    }

    pub fn headers(&self) -> &IndexMap<String, ContentPattern> {
        &self.headers
    }

    pub fn cookies(&self) -> &IndexMap<String, ContentPattern> {
        &self.cookies
    }

    pub fn basic_auth_credentials(&self) -> Option<&BasicCredentials> {
        self.basic_auth_credentials.as_ref()
    }

    pub fn body_patterns(&self) -> &[ContentPattern] {
        &self.body_patterns
    }

    pub fn has_inline_custom_matcher(&self) -> bool {
        // Not yet implemented
        false
    }
}

impl From<RequestPatternBuilder> for RequestPattern {
    fn from(builder: RequestPatternBuilder) -> RequestPattern {
        builder.build()
    }
}

impl<'a> From<RequestPattern> for Cow<'a, RequestPattern> {
    fn from(request_pattern: RequestPattern) -> Self {
        Cow::Owned(request_pattern)
    }
}

impl<'a> From<&'a RequestPattern> for Cow<'a, RequestPattern> {
    fn from(request_pattern: &'a RequestPattern) -> Self {
        Cow::Borrowed(request_pattern)
    }
}

impl fmt::Display for RequestPattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
    }
}
