use std::collections::HashMap;

use reqwest::header::HeaderValue;
use uuid::Uuid;

use crate::client::WireMock;
use crate::common::Metadata;
use crate::extension::Parameters;
use crate::http::ResponseDefinition;
use crate::matching::{ContentPattern, StringValuePattern};
use crate::security::{ClientAuthenticator, NoClientAuthenticator};
use crate::stubbing::StubMapping;

pub struct WireMockBuilder {
    scheme: String,
    host: String,
    port: u16,
    url_path_prefix: String,
    host_header: Option<HeaderValue>,
    authenticator: Box<dyn ClientAuthenticator>,
}

impl WireMockBuilder {
    pub fn new() -> WireMockBuilder {
        WireMockBuilder {
            scheme: "http".to_string(),
            host: "localhost".to_string(),
            port: 8080,
            url_path_prefix: String::new(),
            host_header: None,
            authenticator: Box::new(NoClientAuthenticator),
        }
    }

    pub fn scheme<S: Into<String>>(mut self, scheme: S) -> WireMockBuilder {
        self.scheme = scheme.into();
        self
    }

    pub fn host<S: Into<String>>(mut self, host: S) -> WireMockBuilder {
        self.host = host.into();
        self
    }

    pub fn port(mut self, port: u16) -> WireMockBuilder {
        self.port = port;
        self
    }

    pub fn url_path_prefix<S: Into<String>>(mut self, url_path_prefix: S) -> WireMockBuilder {
        self.url_path_prefix = url_path_prefix.into();
        self
    }

    pub fn host_header<S: Into<String>>(mut self, host_header: S) -> WireMockBuilder {
        self.host_header = Some(host_header.into().parse().unwrap());
        self
    }

    pub fn build(self) -> WireMock {
        WireMock {
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

pub struct MappingBuilder {}

impl MappingBuilder {
    pub fn at_priority(mut self, priority: u16) -> MappingBuilder {
        unimplemented!()
    }

    pub fn with_header<P>(mut self, key: String, header_pattern: P) -> MappingBuilder
        where P: StringValuePattern
    {
        unimplemented!()
    }

    pub fn with_query_param<K, P>(mut self, key: K, query_param_pattern: P) -> MappingBuilder
        where K: Into<String>,
              P: StringValuePattern,
    {
        unimplemented!()
    }

    // fn with_query_params(mut self, Map<String, StringValuePattern> queryParams) -> MappingBuilder;

    pub fn with_request_body<P>(mut self, body_pattern: P) -> MappingBuilder
        where P: Into<ContentPattern>,
    {
        unimplemented!()
    }

    // fn with_multipart_request_body(mut self, MultipartValuePatternBuilder multipartPatternBuilder) -> MappingBuilder;

    pub fn in_scenario<S>(mut self, scenario_name: S) -> ScenarioMappingBuilder
        where S: Into<String>,
    {
        unimplemented!()
    }

    pub fn with_id<U>(mut self, id: Uuid) -> MappingBuilder {
        unimplemented!()
    }

    pub fn with_name<S>(mut self, name: S) -> MappingBuilder
        where S: Into<String>,
    {
        unimplemented!()
    }


    pub fn persistent(mut self) -> MappingBuilder {
        unimplemented!()
    }

    pub fn with_basic_auth<S>(mut self, username: S, password: S) -> MappingBuilder
        where S: Into<String>,
    {
        unimplemented!()
    }


    pub fn with_cookie<S, P>(mut self, name: S, cookie_value_pattern: P) -> MappingBuilder
        where S: Into<String>,
              P: StringValuePattern,
    {
        unimplemented!()
    }


    pub fn with_post_serve_action<S, P>(mut self, extension_name: S, parameters: Parameters)
        -> MappingBuilder
        where S: Into<String>,
    {
        unimplemented!()
    }


    pub fn with_metadata<M>(mut self, metadata: M) -> MappingBuilder
        where M: Into<Metadata>,
    {
        unimplemented!()
    }


//    fn and_matching(mut self, ValueMatcher<Request> requestMatcher) -> MappingBuilder;
//    fn and_matching(mut self, String customRequestMatcherName) -> MappingBuilder;
//    fn and_matching(mut self, String customRequestMatcherName, Parameters parameters) -> MappingBuilder;

    pub fn will_return<R>(mut self, response_definition: R) -> MappingBuilder
        where R: Into<ResponseDefinition>,
    {
        unimplemented!()
    }


    pub fn build(self) -> StubMapping {
        unimplemented!()
    }
}

pub struct ScenarioMappingBuilder;

impl ScenarioMappingBuilder {
    pub fn build(self) -> StubMapping {
        unimplemented!()
    }
}
