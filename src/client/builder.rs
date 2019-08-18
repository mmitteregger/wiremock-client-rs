use reqwest::header::HeaderValue;

use crate::client::WireMockClient;
use crate::security::{ClientAuthenticator, NoClientAuthenticator};

pub struct WireMockClientBuilder {
    scheme: String,
    host: String,
    port: u16,
    url_path_prefix: String,
    host_header: Option<HeaderValue>,
    authenticator: Box<dyn ClientAuthenticator>,
}

impl WireMockClientBuilder {
    pub fn new() -> WireMockClientBuilder {
        WireMockClientBuilder {
            scheme: "http".to_string(),
            host: "localhost".to_string(),
            port: 8080,
            url_path_prefix: String::new(),
            host_header: None,
            authenticator: Box::new(NoClientAuthenticator),
        }
    }

    pub fn scheme<S: Into<String>>(mut self, scheme: S) -> WireMockClientBuilder {
        self.scheme = scheme.into();
        self
    }

    pub fn host<S: Into<String>>(mut self, host: S) -> WireMockClientBuilder {
        self.host = host.into();
        self
    }

    pub fn port(mut self, port: u16) -> WireMockClientBuilder {
        self.port = port;
        self
    }

    pub fn url_path_prefix<S: Into<String>>(mut self, url_path_prefix: S) -> WireMockClientBuilder {
        self.url_path_prefix = url_path_prefix.into();
        self
    }

    pub fn host_header<S: Into<String>>(mut self, host_header: S) -> WireMockClientBuilder {
        self.host_header = Some(host_header.into().parse().unwrap());
        self
    }

    pub fn build(self) -> WireMockClient {
        WireMockClient {
            client: reqwest::Client::new(),
            scheme: self.scheme,
            host: self.host,
            port: self.port,
            url_path_prefix: self.url_path_prefix,
            host_header: self.host_header,
            authenticator: self.authenticator,
        }
    }
}
