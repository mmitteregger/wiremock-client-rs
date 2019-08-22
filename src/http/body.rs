use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Body {
    #[serde(rename = "body")]
    String(String),
    /// The response body as a base64 encoded string (useful for binary content).
    #[serde(rename = "base64Body", with = "crate::serde::base64")]
    Base64(Vec<u8>),
    /// The response body as a JSON object.
    #[serde(rename = "jsonBody")]
    Json(String),
    /// The path to the file containing the response body, relative to the configured file root.
    #[serde(rename = "bodyFileName")]
    FileName(String),
    #[doc(hidden)]
    __Nonexhaustive,
}

impl From<&str> for Body {
    fn from(body: &str) -> Body {
        Body::String(body.to_owned())
    }
}

impl From<String> for Body {
    fn from(body: String) -> Body {
        Body::String(body)
    }
}

impl From<Vec<u8>> for Body {
    fn from(body: Vec<u8>) -> Body {
        Body::Base64(body)
    }
}
