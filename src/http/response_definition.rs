use http::HeaderMap;
use serde::{Deserialize, Serialize};

use crate::client::{ProxyResponseDefinitionBuilder, ResponseDefinitionBuilder};
use crate::extension::Parameters;
use crate::http::{Body, ChunkedDribbleDelay, DelayDistribution, Fault};

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
    #[serde(default, skip_serializing_if = "HeaderMap::is_empty", with = "crate::serde::header_map")]
    pub(crate) headers: HeaderMap,
    /// Map of additional response headers to send when proxying.
    #[serde(rename = "additionalProxyRequestHeaders", default, skip_serializing_if = "HeaderMap::is_empty", with = "crate::serde::header_map")]
    pub(crate) additional_proxy_request_headers: HeaderMap,
    /// Number of milliseconds to delay be before sending the response.
    #[serde(rename = "fixedDelayMilliseconds", skip_serializing_if = "Option::is_none")]
    pub(crate) fixed_delay_milliseconds: Option<u32>,
    #[serde(rename = "delayDistribution", skip_serializing_if = "Option::is_none")]
    pub(crate) delay_distribution: Option<DelayDistribution>,
    #[serde(rename = "chunkedDribbleDelay", skip_serializing_if = "Option::is_none")]
    pub(crate) chunked_dribble_delay: Option<ChunkedDribbleDelay>,
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

    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    pub fn additional_proxy_request_headers(&self) -> &HeaderMap {
        &self.additional_proxy_request_headers
    }

    pub fn fixed_delay_milliseconds(&self) -> Option<u32> {
        self.fixed_delay_milliseconds
    }

    pub fn delay_distribution(&self) -> Option<&DelayDistribution> {
        self.delay_distribution.as_ref()
    }

    pub fn chunked_dribble_delay(&self) -> Option<&ChunkedDribbleDelay> {
        self.chunked_dribble_delay.as_ref()
    }

    pub fn proxy_base_url(&self) -> Option<&str> {
        self.proxy_base_url.as_ref().map(|proxy_base_url| proxy_base_url.as_str())
    }

    pub fn fault(&self) -> Option<Fault> {
        self.fault
    }

    pub fn transformers(&self) -> &[String] {
        &self.transformers
    }

    pub fn transformer_parameters(&self) -> &Parameters {
        &self.transformer_parameters
    }
}

impl From<ResponseDefinitionBuilder> for ResponseDefinition {
    fn from(builder: ResponseDefinitionBuilder) -> ResponseDefinition {
        builder.build()
    }
}

impl From<ProxyResponseDefinitionBuilder> for ResponseDefinition {
    fn from(builder: ProxyResponseDefinitionBuilder) -> ResponseDefinition {
        builder.build()
    }
}
