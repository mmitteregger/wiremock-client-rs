use serde::{Deserialize, Serialize};
use indexmap::IndexMap;

pub use crate::http::delay_distribution::DelayDistribution;
use crate::client::builder::ResponseDefinitionBuilder;
use crate::extension::Parameters;

mod delay_distribution;

pub type Result<T> = reqwest::Result<T>;

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub enum RequestMethod {
    ANY,
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    CONNECT,
    PATCH,
    TRACE,
    #[doc(hidden)]
    __Nonexhaustive,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseDefinition {
    /// The HTTP status code to be returned.
    pub(crate) status: u16,
    /// The HTTP status message to be returned.
    #[serde(rename = "statusMessage", skip_serializing_if = "Option::is_none")]
    pub(crate) status_message: Option<String>,
    /// The response body to be returned.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub(crate) body: Option<Body>,
    /// Map of response headers to send.
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub(crate) headers: IndexMap<String, String>,
    /// Number of milliseconds to delay be before sending the response.
    #[serde(rename = "fixedDelayMilliseconds", skip_serializing_if = "Option::is_none")]
    pub(crate) fixed_delay_milliseconds: Option<u32>,
    #[serde(rename = "delayDistribution", skip_serializing_if = "Option::is_none")]
    pub(crate) delay_distribution: Option<DelayDistribution>,
    /// The base URL of the target to proxy matching requests to.
    #[serde(rename = "proxyBaseUrl", skip_serializing_if = "Option::is_none")]
    pub(crate) proxy_base_url: Option<String>,
    /// The fault to apply (instead of a full, valid response).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) fault: Option<Fault>,
    /// List of names of transformers to apply to this response.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(crate) transformers: Vec<String>,
    /// Parameters to apply to response transformers.
    #[serde(rename = "transformerParameters", flatten, default, skip_serializing_if = "Parameters::is_empty")]
    pub(crate) transformer_parameters: Parameters,
    /// Read-only flag indicating false if this was the default, unmatched response. Not present otherwise.
    #[serde(rename = "fromConfiguredStub", default = "crate::serde::default_true", skip_serializing)]
    pub(crate) from_configured_stub: bool,
}

impl ResponseDefinition {
    pub fn status(&self) -> u16 {
        self.status
    }

    pub fn status_message(&self) -> Option<&str> {
        self.status_message.as_ref().map(|status_message| status_message.as_str())
    }

    pub fn body(&self) -> Option<&Body> {
        self.body.as_ref()
    }

    pub fn headers(&self) -> &IndexMap<String, String> {
        &self.headers
    }

    pub fn headers_mut(&mut self) -> &mut IndexMap<String, String> {
        &mut self.headers
    }

    pub fn fixed_delay_milliseconds(&self) -> Option<u32> {
        self.fixed_delay_milliseconds
    }

    pub fn delay_distribution(&self) -> Option<&DelayDistribution> {
        self.delay_distribution.as_ref()
    }

    pub fn proxy_base_url(&self) -> Option<&str> {
        self.proxy_base_url.as_ref().map(|proxy_base_url| proxy_base_url.as_str())
    }

    pub fn fault(&self) -> Option<&Fault> {
        self.fault.as_ref()
    }

    pub fn transformers(&self) -> &[String] {
        &self.transformers
    }

    pub fn transformers_mut(&mut self) -> &mut Vec<String> {
        &mut self.transformers
    }

    pub fn transformer_parameters(&self) -> &Parameters {
        &self.transformer_parameters
    }

    pub fn transformer_parameters_mut(&mut self) -> &mut Parameters {
        &mut self.transformer_parameters
    }
}

impl From<ResponseDefinitionBuilder> for ResponseDefinition {
    fn from(builder: ResponseDefinitionBuilder) -> ResponseDefinition {
        builder.build()
    }
}

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

#[derive(Debug, Serialize, Deserialize)]
pub struct ChunkedDribbleDelay {
    number_of_chunks: u16,
    total_duration: u16,
}

impl ChunkedDribbleDelay {
    pub fn new(number_of_chunks: u16, total_duration: u16) -> ChunkedDribbleDelay {
        ChunkedDribbleDelay {
            number_of_chunks,
            total_duration,
        }
    }

    pub fn number_of_chunks(&self) -> u16 {
        self.number_of_chunks
    }

    pub fn total_duration(&self) -> u16 {
        self.total_duration
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
/// The fault to apply (instead of a full, valid response).
pub enum Fault {
    CONNECTION_RESET_BY_PEER,
    EMPTY_RESPONSE,
    MALFORMED_RESPONSE_CHUNK,
    RANDOM_DATA_THEN_CLOSE,
    #[doc(hidden)]
    __Nonexhaustive,
}
