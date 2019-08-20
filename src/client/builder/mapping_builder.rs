use uuid::Uuid;
use indexmap::IndexMap;

use crate::common::Metadata;
use crate::extension::Parameters;
use crate::http::{ResponseDefinition, RequestMethod};
use crate::matching::{ContentPattern, StringValuePattern, UrlPattern, RequestPatternBuilder};
use crate::stubbing::StubMapping;
use crate::client::BasicCredentials;

pub struct MappingBuilder {
    request_pattern_builder: RequestPatternBuilder,
    response_definition: Option<ResponseDefinition>,
    priority: Option<u16>,
    id: Uuid,
    name: Option<String>,
    persistent: bool,
    post_serve_actions: IndexMap<String, Parameters>,
    metadata: Metadata,
}

impl MappingBuilder {
    pub(crate) fn new(request_method: RequestMethod, url_pattern: UrlPattern) -> MappingBuilder {
        MappingBuilder {
            request_pattern_builder: RequestPatternBuilder::new(request_method, url_pattern),
            response_definition: None,
            priority: None,
            id: Uuid::new_v4(),
            name: None,
            persistent: false,
            post_serve_actions: Default::default(),
            metadata: Metadata::new(),
        }
    }

    fn with_request_pattern_builder(request_pattern_builder: RequestPatternBuilder) -> MappingBuilder {
        MappingBuilder {
            request_pattern_builder,
            response_definition: None,
            priority: None,
            id: Uuid::new_v4(),
            name: None,
            persistent: false,
            post_serve_actions: Default::default(),
            metadata: Metadata::new(),
        }
    }

    pub fn at_priority(mut self, priority: u16) -> MappingBuilder {
        self.priority = Some(priority);
        self
    }

    pub fn with_header<S, P>(mut self, key: S, header_pattern: P) -> MappingBuilder
        where S: Into<String>,
              P: StringValuePattern,
    {
        self.do_with_request_pattern_builder(|builder|
            builder.with_header(key, header_pattern));
        self
    }

    pub fn with_query_param<K, P>(mut self, key: K, query_param_pattern: P) -> MappingBuilder
        where K: Into<String>,
              P: StringValuePattern,
    {
        self.do_with_request_pattern_builder(|builder|
            builder.with_query_param(key, query_param_pattern));
        self
    }

    // fn with_query_params(mut self, Map<String, StringValuePattern> queryParams) -> MappingBuilder;

    pub fn with_request_body<P>(mut self, body_pattern: P) -> MappingBuilder
        where P: Into<ContentPattern>,
    {
        self.do_with_request_pattern_builder(|builder|
            builder.with_request_body(body_pattern));
        self
    }

    // fn with_multipart_request_body(mut self, MultipartValuePatternBuilder multipartPatternBuilder) -> MappingBuilder;

    pub fn in_scenario<S>(self, scenario_name: S) -> ScenarioMappingBuilder
        where S: Into<String>,
    {
        ScenarioMappingBuilder {
            mapping_builder: self,
            scenario_name: scenario_name.into(),
            required_scenario_state: None,
            new_scenario_state: None,
        }
    }

    pub fn with_id(mut self, id: Uuid) -> MappingBuilder {
        self.id = id;
        self
    }

    pub fn with_name<S>(mut self, name: S) -> MappingBuilder
        where S: Into<String>,
    {
        self.name = Some(name.into());
        self
    }

    pub fn persistent(mut self) -> MappingBuilder {
        self.persistent = true;
        self
    }

    pub fn with_basic_auth<S>(mut self, username: S, password: S) -> MappingBuilder
        where S: Into<String>,
    {
        self.do_with_request_pattern_builder(|builder|
            builder.with_basic_auth(BasicCredentials {
                username: username.into(),
                password: password.into(),
            }));
        self
    }

    pub fn with_cookie<S, P>(mut self, name: S, cookie_value_pattern: P) -> MappingBuilder
        where S: Into<String>,
              P: StringValuePattern,
    {
        self.do_with_request_pattern_builder(|builder|
            builder.with_cookie(name, cookie_value_pattern));
        self
    }

    pub fn with_post_serve_action<S, P>(mut self, extension_name: S, parameters: P)
        -> MappingBuilder
        where S: Into<String>,
              P: Into<Parameters>,
    {
        self.post_serve_actions.insert(extension_name.into(), parameters.into());
        self
    }

    pub fn with_metadata<M>(mut self, metadata: M) -> MappingBuilder
        where M: Into<Metadata>,
    {
        self.metadata = metadata.into();
        self
    }

//    fn and_matching(mut self, ValueMatcher<Request> requestMatcher) -> MappingBuilder;
//    fn and_matching(mut self, String customRequestMatcherName) -> MappingBuilder;
//    fn and_matching(mut self, String customRequestMatcherName, Parameters parameters) -> MappingBuilder;

    pub fn will_return<R>(mut self, response_definition: R) -> MappingBuilder
        where R: Into<ResponseDefinition>,
    {
        self.response_definition = Some(response_definition.into());
        self
    }

    pub fn build(self) -> StubMapping {
        StubMapping {
            id: self.id,
            name: self.name,
            request: self.request_pattern_builder.build(),
            response: self.response_definition
                .unwrap_or_else(|| crate::a_response().build()),
            persistent: Some(self.persistent),
            priority: self.priority,
            scenario_name: None,
            required_scenario_state: None,
            new_scenario_state: None,
            post_serve_actions: self.post_serve_actions,
            metadata: self.metadata,
        }
    }

    fn do_with_request_pattern_builder<F>(&mut self, closure: F)
        where F: FnOnce(RequestPatternBuilder) -> RequestPatternBuilder
    {
        let default_builder = RequestPatternBuilder::all_requests();
        let builder = std::mem::replace(&mut self.request_pattern_builder, default_builder);
        let new_builder = closure(builder);
        self.request_pattern_builder = new_builder;
    }
}

pub struct ScenarioMappingBuilder {
    mapping_builder: MappingBuilder,
    scenario_name: String,
    required_scenario_state: Option<String>,
    new_scenario_state: Option<String>,
}

impl ScenarioMappingBuilder {
    pub fn when_scenario_state_is<S>(mut self, state_name: S) -> ScenarioMappingBuilder
        where S: Into<String>
    {
        self.required_scenario_state = Some(state_name.into());
        self
    }

    pub fn will_set_state_to<S>(mut self, state_name: S) -> ScenarioMappingBuilder
        where S: Into<String>
    {
        self.new_scenario_state = Some(state_name.into());
        self
    }

    pub fn at_priority(mut self, priority: u16) -> ScenarioMappingBuilder {
        self.do_with_mapping_builder(|builder|
            builder.at_priority(priority));
        self
    }

    pub fn with_header<S, P>(mut self, key: S, header_pattern: P) -> ScenarioMappingBuilder
        where S: Into<String>,
              P: StringValuePattern,
    {
        self.do_with_mapping_builder(|builder|
            builder.with_header(key, header_pattern));
        self
    }

    pub fn with_query_param<K, P>(mut self, key: K, query_param_pattern: P) -> ScenarioMappingBuilder
        where K: Into<String>,
              P: StringValuePattern,
    {
        self.do_with_mapping_builder(|builder|
            builder.with_query_param(key, query_param_pattern));
        self
    }

    // fn with_query_params(mut self, Map<String, StringValuePattern> queryParams) -> ScenarioMappingBuilder;

    pub fn with_request_body<P>(mut self, body_pattern: P) -> ScenarioMappingBuilder
        where P: Into<ContentPattern>,
    {
        self.do_with_mapping_builder(|builder|
            builder.with_request_body(body_pattern));
        self
    }

    // fn with_multipart_request_body(mut self, MultipartValuePatternBuilder multipartPatternBuilder) -> ScenarioMappingBuilder;

    pub fn in_scenario<S>(mut self, scenario_name: S) -> ScenarioMappingBuilder
        where S: Into<String>,
    {
        self.scenario_name = scenario_name.into();
        self
    }

    pub fn with_id(mut self, id: Uuid) -> ScenarioMappingBuilder {
        self.do_with_mapping_builder(|builder|
            builder.with_id(id));
        self
    }

    pub fn with_name<S>(mut self, name: S) -> ScenarioMappingBuilder
        where S: Into<String>,
    {
        self.do_with_mapping_builder(|builder|
            builder.with_name(name));
        self
    }

    pub fn persistent(mut self) -> ScenarioMappingBuilder {
        self.do_with_mapping_builder(|builder|
            builder.persistent());
        self
    }

    pub fn with_basic_auth<S>(mut self, username: S, password: S) -> ScenarioMappingBuilder
        where S: Into<String>,
    {
        self.do_with_mapping_builder(|builder|
            builder.with_basic_auth(username, password));
        self
    }

    pub fn with_cookie<S, P>(mut self, name: S, cookie_value_pattern: P) -> ScenarioMappingBuilder
        where S: Into<String>,
              P: StringValuePattern,
    {
        self.do_with_mapping_builder(|builder|
            builder.with_cookie(name, cookie_value_pattern));
        self
    }

    pub fn with_post_serve_action<S, P>(mut self, extension_name: S, parameters: Parameters)
        -> ScenarioMappingBuilder
        where S: Into<String>,
    {
        self.do_with_mapping_builder(|builder|
            builder.with_post_serve_action(extension_name, parameters));
        self
    }

    pub fn with_metadata<M>(mut self, metadata: M) -> ScenarioMappingBuilder
        where M: Into<Metadata>,
    {
        self.do_with_mapping_builder(|builder|
            builder.with_metadata(metadata));
        self
    }

//    fn and_matching(mut self, ValueMatcher<Request> requestMatcher) -> ScenarioMappingBuilder;
//    fn and_matching(mut self, String customRequestMatcherName) -> ScenarioMappingBuilder;
//    fn and_matching(mut self, String customRequestMatcherName, Parameters parameters) -> ScenarioMappingBuilder;

    pub fn will_return<R>(mut self, response_definition: R) -> ScenarioMappingBuilder
        where R: Into<ResponseDefinition>,
    {
        self.do_with_mapping_builder(|builder|
            builder.will_return(response_definition));
        self
    }

    pub fn build(self) -> StubMapping {
        let mut stub_mapping = self.mapping_builder.build();
        stub_mapping.scenario_name = Some(self.scenario_name);
        stub_mapping.required_scenario_state = self.required_scenario_state;
        stub_mapping.new_scenario_state = self.new_scenario_state;
        stub_mapping
    }

    fn do_with_mapping_builder<F>(&mut self, closure: F)
        where F: FnOnce(MappingBuilder) -> MappingBuilder
    {
        let default_builder = MappingBuilder::with_request_pattern_builder(RequestPatternBuilder::all_requests());
        let builder = std::mem::replace(&mut self.mapping_builder, default_builder);
        let new_builder = closure(builder);
        self.mapping_builder = new_builder;
    }
}
