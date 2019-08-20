use indexmap::IndexMap;

use crate::http::{ResponseDefinition, Body, Fault, DelayDistribution};
use reqwest::StatusCode;

pub struct ResponseDefinitionBuilder {
    status: u16,
    status_message: Option<String>,
    body: Option<Body>,
    headers: IndexMap<String, String>,
    fixed_delay_milliseconds: Option<u32>,
    delay_distribution: Option<DelayDistribution>,
    proxy_base_url: Option<String>,
    fault: Option<Fault>,
    transformers: Vec<String>,
    transformer_parameters: IndexMap<String, serde_json::Value>,
    from_configured_stub: bool,
}

impl ResponseDefinitionBuilder {
    pub(crate) fn new() -> ResponseDefinitionBuilder {
        ResponseDefinitionBuilder {
            status: StatusCode::OK.as_u16(),
            status_message: None,
            body: None,
            headers: IndexMap::new(),
            fixed_delay_milliseconds: None,
            delay_distribution: None,
            proxy_base_url: None,
            fault: None,
            transformers: Vec::new(),
            transformer_parameters: IndexMap::new(),
            from_configured_stub: true,
        }
    }

    pub fn with_status(mut self, status: u16) -> ResponseDefinitionBuilder {
        self.status = status;
        self
    }

    pub fn with_header<K, V>(mut self, key: K, value: V) -> ResponseDefinitionBuilder
        where K: Into<String>,
              V: Into<String>,
    {
        self.headers.insert(key.into(), value.into());
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

//    pub fn with_chunked_dribble_delay(mut self, number_of_chunks: u16, total_duration: u16) -> ResponseDefinitionBuilder {
//        self.chunkedDribbleDelay = new ChunkedDribbleDelay(number_of_chunks, total_duration);
//        self
//    }

    pub fn with_transformer<S>(mut self, response_transformer_name: S) -> ResponseDefinitionBuilder
        where S: Into<String>,
    {
        self.transformers.push(response_transformer_name.into());
        self
    }

    pub fn with_transformers<I>(mut self, response_transformer_names: I) -> ResponseDefinitionBuilder
        where I: IntoIterator<Item=String>,
    {
        self.transformers.extend(response_transformer_names.into_iter());
        self
    }

    pub fn with_transformer_parameter<K, V>(mut self, key: K, value: V) -> ResponseDefinitionBuilder
        where K: Into<String>,
              V: Into<serde_json::Value>,
    {
        self.transformer_parameters.insert(key.into(), value.into());
        self
    }

//    pub fn proxied_from(mut self, proxy_base_url: S) -> ProxyResponseDefinitionBuilder
//        where S: Into<String>
//    {
//        self.proxy_base_url = Some(proxy_base_url.into());
//        return new ProxyResponseDefinitionBuilder(this);
//    }

    pub fn proxied_from<S>(mut self, proxy_base_url: S) -> ResponseDefinitionBuilder
        where S: Into<String>,
    {
        self.proxy_base_url = Some(proxy_base_url.into());
        self
    }

//    public static ResponseDefinitionBuilder responseDefinition() {
//        return new ResponseDefinitionBuilder();
//    }
//
//    public static <T> ResponseDefinitionBuilder okForJson(T body) {
//        return responseDefinition().withStatus(HTTP_OK).withBody(Json.write(body)).withHeader("Content-Type", "application/json");
//    }
//
//    public static <T> ResponseDefinitionBuilder okForEmptyJson() {
//        return responseDefinition().withStatus(HTTP_OK).withBody("{}").withHeader("Content-Type", "application/json");
//    }

    pub fn with_headers(mut self, headers: IndexMap<String, String>) -> ResponseDefinitionBuilder {
        self.headers = headers;
        self
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
            fixed_delay_milliseconds: self.fixed_delay_milliseconds,
            delay_distribution: self.delay_distribution,
            proxy_base_url: self.proxy_base_url,
            fault: self.fault,
            transformers: self.transformers,
            transformer_parameters: self.transformer_parameters,
            from_configured_stub: self.from_configured_stub,
        }
    }
}
