use http::{HttpTryFrom, StatusCode};
use http::header::{HeaderMap, HeaderName, HeaderValue};

use crate::extension::Parameters;
use crate::http::{Body, DelayDistribution, Fault, ResponseDefinition, ChunkedDribbleDelay};

pub struct ResponseDefinitionBuilder {
    status: u16,
    status_message: Option<String>,
    body: Option<Body>,
    headers: HeaderMap,
    fixed_delay_milliseconds: Option<u32>,
    delay_distribution: Option<DelayDistribution>,
    chunked_dribble_delay: Option<ChunkedDribbleDelay>,
    proxy_base_url: Option<String>,
    fault: Option<Fault>,
    transformers: Vec<String>,
    transformer_parameters: Parameters,
    from_configured_stub: bool,
}

impl ResponseDefinitionBuilder {
    pub(crate) fn new() -> ResponseDefinitionBuilder {
        ResponseDefinitionBuilder {
            status: StatusCode::OK.as_u16(),
            status_message: None,
            body: None,
            headers: HeaderMap::new(),
            fixed_delay_milliseconds: None,
            delay_distribution: None,
            chunked_dribble_delay: None,
            proxy_base_url: None,
            fault: None,
            transformers: Vec::new(),
            transformer_parameters: Parameters::empty(),
            from_configured_stub: true,
        }
    }

    pub fn with_status<S>(mut self, status: S) -> ResponseDefinitionBuilder
        where StatusCode: HttpTryFrom<S>,
    {
        let status_code = match StatusCode::try_from(status) {
            Ok(status_code) => status_code,
            Err(_) => panic!("invalid status code"),
        };
        self.status = status_code.as_u16();
        self
    }

    pub fn with_header<K, V>(mut self, key: K, value: V) -> ResponseDefinitionBuilder
        where K: AsRef<str>,
              V: AsRef<str>,
    {
        let key_ref = key.as_ref();
        let header_name = match HeaderName::try_from(key_ref) {
            Ok(name) => name,
            Err(_invalid_header_name_error) => {
                panic!("header name \"{}\" is invalid", key_ref);
            }
        };

        let value_ref = value.as_ref();
        let header_value = match HeaderValue::from_str(value_ref) {
            Ok(name) => name,
            Err(_invalid_header_value_error) => {
                panic!("header value \"{}\" is invalid", value_ref);
            }
        };

        self.headers.append(header_name, header_value);
        self
    }

    pub fn with_headers<'a, H>(mut self, headers: H) -> ResponseDefinitionBuilder
        where H: Into<HeaderMap>,
    {
        self.headers = headers.into();
        self
    }

    pub fn with_body_file<S>(mut self, file_name: S) -> ResponseDefinitionBuilder
        where S: Into<String>,
    {
        self.body = Some(Body::FileName(file_name.into()));
        self
    }

    pub fn with_body<B>(mut self, body: B) -> ResponseDefinitionBuilder
        where B: Into<Body>,
    {
        self.body = Some(body.into());
        self
    }

    pub fn with_fixed_delay(mut self, milliseconds: u32) -> ResponseDefinitionBuilder {
        self.fixed_delay_milliseconds = Some(milliseconds);
        self
    }

    pub fn with_random_delay(mut self, distribution: DelayDistribution) -> ResponseDefinitionBuilder {
        self.delay_distribution = Some(distribution);
        self
    }

    pub fn with_log_normal_random_delay(self, median_millis: f64, sigma: f64) -> ResponseDefinitionBuilder {
        self.with_random_delay(DelayDistribution::LogNormal {
            median: median_millis,
            sigma,
        })
    }

    pub fn with_uniform_random_delay(self, lower_millis: u32, upper_millis: u32) -> ResponseDefinitionBuilder {
        self.with_random_delay(DelayDistribution::Uniform {
            lower: lower_millis,
            upper: upper_millis,
        })
    }

    pub fn with_chunked_dribble_delay(mut self, number_of_chunks: u16, total_duration: u16) -> ResponseDefinitionBuilder {
        self.chunked_dribble_delay = Some(ChunkedDribbleDelay { number_of_chunks, total_duration });
        self
    }

    pub fn with_transformer<S>(mut self, response_transformer_name: S) -> ResponseDefinitionBuilder
        where S: Into<String>,
    {
        self.transformers.push(response_transformer_name.into());
        self
    }

    pub fn with_transformers<I>(mut self, response_transformer_names: I) -> ResponseDefinitionBuilder
        where I: IntoIterator<Item=String>,
    {
        self.transformers.extend(response_transformer_names);
        self
    }

    pub fn with_transformer_parameter<K, V>(mut self, key: K, value: V) -> ResponseDefinitionBuilder
        where K: Into<String>,
              V: Into<serde_json::Value>,
    {
        self.transformer_parameters.parameters.insert(key.into(), value.into());
        self
    }

    pub fn proxied_from<S>(mut self, proxy_base_url: S) -> ProxyResponseDefinitionBuilder
        where S: Into<String>,
    {
        self.proxy_base_url = Some(proxy_base_url.into());
        ProxyResponseDefinitionBuilder {
            response_definition_builder: self,
            additional_request_headers: HeaderMap::new(),
        }
    }

    pub fn with_base64body<B>(mut self, base64_body: B) -> ResponseDefinitionBuilder
        where B: Into<Vec<u8>>,
    {
        self.body = Some(Body::Base64(base64_body.into()));
        self
    }

    pub fn with_status_message<S>(mut self, message: S) -> ResponseDefinitionBuilder
        where S: Into<String>,
    {
        self.status_message = Some(message.into());
        self
    }

    pub fn with_fault(mut self, fault: Fault) -> ResponseDefinitionBuilder {
        self.fault = Some(fault);
        self
    }

    pub fn build(self) -> ResponseDefinition {
        ResponseDefinition {
            status: self.status,
            status_message: self.status_message,
            body: self.body,
            headers: self.headers,
            additional_proxy_request_headers: HeaderMap::new(),
            fixed_delay_milliseconds: self.fixed_delay_milliseconds,
            delay_distribution: self.delay_distribution,
            chunked_dribble_delay: self.chunked_dribble_delay,
            proxy_base_url: self.proxy_base_url,
            fault: self.fault,
            transformers: self.transformers,
            transformer_parameters: self.transformer_parameters,
            from_configured_stub: self.from_configured_stub,
        }
    }
}

pub struct ProxyResponseDefinitionBuilder {
    response_definition_builder: ResponseDefinitionBuilder,
    additional_request_headers: HeaderMap,
}

impl ProxyResponseDefinitionBuilder {
    pub fn with_additional_request_header<K, V>(mut self, key: K, value: V) -> ProxyResponseDefinitionBuilder
        where K: AsRef<str>,
              V: AsRef<str>,
    {
        let key_ref = key.as_ref();
        let header_name = match HeaderName::try_from(key_ref) {
            Ok(name) => name,
            Err(_invalid_header_name_error) => {
                panic!("header name \"{}\" is invalid", key_ref);
            }
        };

        let value_ref = value.as_ref();
        let header_value = match HeaderValue::from_str(value_ref) {
            Ok(name) => name,
            Err(_invalid_header_value_error) => {
                panic!("header value \"{}\" is invalid", value_ref);
            }
        };

        self.additional_request_headers.append(header_name, header_value);
        self
    }

    pub fn with_additional_request_headers<'a, H>(mut self, headers: H) -> ProxyResponseDefinitionBuilder
        where H: Into<HeaderMap>,
    {
        self.additional_request_headers = headers.into();
        self
    }

    pub fn with_status<S>(mut self, status: S) -> ProxyResponseDefinitionBuilder
        where StatusCode: HttpTryFrom<S>,
    {
        self.do_with_response_definition_builder(|builder|
            builder.with_status(status));
        self
    }

    pub fn with_header<K, V>(mut self, key: K, value: V) -> ProxyResponseDefinitionBuilder
        where K: AsRef<str>,
              V: AsRef<str>,
    {
        self.do_with_response_definition_builder(|builder|
            builder.with_header(key, value));
        self
    }

    pub fn with_headers<'a, H>(mut self, headers: H) -> ProxyResponseDefinitionBuilder
        where H: Into<HeaderMap>,
    {
        self.do_with_response_definition_builder(|builder|
            builder.with_headers(headers));
        self
    }

    pub fn with_body_file<S>(mut self, file_name: S) -> ProxyResponseDefinitionBuilder
        where S: Into<String>,
    {
        self.do_with_response_definition_builder(|builder|
            builder.with_body_file(file_name));
        self
    }

    pub fn with_body<B>(mut self, body: B) -> ProxyResponseDefinitionBuilder
        where B: Into<Body>,
    {
        self.do_with_response_definition_builder(|builder|
            builder.with_body(body));
        self
    }

    pub fn with_fixed_delay(mut self, milliseconds: u32) -> ProxyResponseDefinitionBuilder {
        self.do_with_response_definition_builder(|builder|
            builder.with_fixed_delay(milliseconds));
        self
    }

    pub fn with_random_delay(mut self, distribution: DelayDistribution) -> ProxyResponseDefinitionBuilder {
        self.do_with_response_definition_builder(|builder|
            builder.with_random_delay(distribution));
        self
    }

    pub fn with_log_normal_random_delay(mut self, median_millis: f64, sigma: f64) -> ProxyResponseDefinitionBuilder {
        self.do_with_response_definition_builder(|builder|
            builder.with_log_normal_random_delay(median_millis, sigma));
        self
    }

    pub fn with_uniform_random_delay(mut self, lower_millis: u32, upper_millis: u32) -> ProxyResponseDefinitionBuilder {
        self.do_with_response_definition_builder(|builder|
            builder.with_uniform_random_delay(lower_millis, upper_millis));
        self
    }

    pub fn with_chunked_dribble_delay(mut self, number_of_chunks: u16, total_duration: u16) -> ProxyResponseDefinitionBuilder {
        self.do_with_response_definition_builder(|builder|
            builder.with_chunked_dribble_delay(number_of_chunks, total_duration));
        self
    }

    pub fn with_transformer<S>(mut self, response_transformer_name: S) -> ProxyResponseDefinitionBuilder
        where S: Into<String>,
    {
        self.do_with_response_definition_builder(|builder|
            builder.with_transformer(response_transformer_name));
        self
    }

    pub fn with_transformers<I>(mut self, response_transformer_names: I) -> ProxyResponseDefinitionBuilder
        where I: IntoIterator<Item=String>,
    {
        self.do_with_response_definition_builder(|builder|
            builder.with_transformers(response_transformer_names));
        self
    }

    pub fn with_transformer_parameter<K, V>(mut self, key: K, value: V) -> ProxyResponseDefinitionBuilder
        where K: Into<String>,
              V: Into<serde_json::Value>,
    {
        self.do_with_response_definition_builder(|builder|
            builder.with_transformer_parameter(key, value));
        self
    }

    pub fn with_base64body<B>(mut self, base64_body: B) -> ProxyResponseDefinitionBuilder
        where B: Into<Vec<u8>>,
    {
        self.do_with_response_definition_builder(|builder|
            builder.with_base64body(base64_body));
        self
    }

    pub fn with_status_message<S>(mut self, message: S) -> ProxyResponseDefinitionBuilder
        where S: Into<String>,
    {
        self.do_with_response_definition_builder(|builder|
            builder.with_status_message(message));
        self
    }

    pub fn with_fault(mut self, fault: Fault) -> ProxyResponseDefinitionBuilder {
        self.do_with_response_definition_builder(|builder|
            builder.with_fault(fault));
        self
    }

    pub fn build(self) -> ResponseDefinition {
        ResponseDefinition {
            status: self.response_definition_builder.status,
            status_message: self.response_definition_builder.status_message,
            body: self.response_definition_builder.body,
            headers: self.response_definition_builder.headers,
            additional_proxy_request_headers: self.additional_request_headers,
            fixed_delay_milliseconds: self.response_definition_builder.fixed_delay_milliseconds,
            delay_distribution: self.response_definition_builder.delay_distribution,
            chunked_dribble_delay: self.response_definition_builder.chunked_dribble_delay,
            proxy_base_url: self.response_definition_builder.proxy_base_url,
            fault: self.response_definition_builder.fault,
            transformers: self.response_definition_builder.transformers,
            transformer_parameters: self.response_definition_builder.transformer_parameters,
            from_configured_stub: self.response_definition_builder.from_configured_stub,
        }
    }

    fn do_with_response_definition_builder<F>(&mut self, closure: F)
        where F: FnOnce(ResponseDefinitionBuilder) -> ResponseDefinitionBuilder
    {
        let default_builder = ResponseDefinitionBuilder::new();
        let builder = std::mem::replace(&mut self.response_definition_builder, default_builder);
        let new_builder = closure(builder);
        self.response_definition_builder = new_builder;
    }
}
