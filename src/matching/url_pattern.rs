use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
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

impl From<&String> for UrlPattern {
    fn from(url: &String) -> UrlPattern {
        UrlPattern::Url(url.clone())
    }
}

impl From<String> for UrlPattern {
    fn from(url: String) -> UrlPattern {
        UrlPattern::Url(url)
    }
}
