use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::client::BasicCredentials;
use crate::http::RequestMethod;

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestPattern {
    /// The url pattern to match exactly against.
    pub url: Url,
    /// The HTTP request method e.g. GET
    pub method: RequestMethod,
    /// Query parameter patterns to match against.
    #[serde(rename = "queryParameters", default, skip_serializing_if = "HashMap::is_empty")]
    pub query_params: HashMap<String, String>,
    /// Header patterns to match against.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub headers: HashMap<String, String>,
    /// Cookie patterns to match against.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub cookies: HashMap<String, String>,
    /// Pre-emptive basic auth credentials to match against.
    #[serde(rename = "basicAuthCredentials")]
    pub basic_auth_credentials: Option<BasicCredentials>,
    /// Request body patterns to match against.
    #[serde(rename = "bodyPatterns", default, skip_serializing_if = "Vec::is_empty")]
    pub body_patterns: Vec<ContentPattern>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Url {
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContentPattern {
    Absent {
        absent: String,
    },
    Anything {
        anything: String,
    },
    BinaryEqualTo {
        /// Base64 encoded string.
        #[serde(rename = "binaryEqualTo", with = "crate::serde::base64")]
        binary_equal_to: Vec<u8>,
    },
    Contains {
        contains: String,
    },
    EqualToJson {
        #[serde(rename = "equalToJson")]
        equal_to_json: String,
        #[serde(rename = "ignoreArrayOrder")]
        ignore_array_order: bool,
        #[serde(rename = "ignoreExtraElements")]
        ignore_extra_elements: bool,
    },
    EqualTo {
        #[serde(rename = "equalTo")]
        equal_to: String,
        #[serde(rename = "caseInsensitive")]
        case_insensitive: bool,
    },
    EqualToXml {
        #[serde(rename = "equalToXml")]
        equal_to_xml: String,
        #[serde(rename = "enablePlaceholders")]
        enable_placeholders: Option<bool>,
        #[serde(rename = "placeholderOpeningDelimiterRegex")]
        placeholder_opening_delimiter_regex: Option<String>,
        #[serde(rename = "placeholderClosingDelimiterRegex")]
        placeholder_closing_delimiter_regex: Option<String>,
    },
//    MatchesJsonPath,
//    MatchesXPath,
    Regex {
        matches: String,
    },
    NegativeRegex {
        #[serde(rename = "doesNotMatch")]
        does_not_match: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultipartValuePattern;

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiValuePattern {
    value_pattern: StringValuePattern,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StringValuePattern {
    value: Option<String>,
}
