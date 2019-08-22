use crate::matching::{UrlPattern, ContentPattern, RequestPattern, StringValuePattern, AbsentPattern};
use crate::http::RequestMethod;
use indexmap::IndexMap;
use crate::BasicCredentials;

pub struct RequestPatternBuilder {
    method: RequestMethod,
    url_pattern: UrlPattern,
    headers: IndexMap<String, ContentPattern>,
    query_params: IndexMap<String, ContentPattern>,
    body_patterns: Vec<ContentPattern>,
    cookies: IndexMap<String, ContentPattern>,
    basic_credentials: Option<BasicCredentials>,
//    multiparts: Vec<MultipartValuePattern>,
//    custom_matcher: ValueMatcher<Request>,
//    custom_matcher_definition: CustomMatcherDefinition,
}

impl RequestPatternBuilder {
    pub(crate) fn new(method: RequestMethod, url_pattern: UrlPattern) -> RequestPatternBuilder {
        RequestPatternBuilder {
            method,
            url_pattern,
            headers: IndexMap::new(),
            query_params: IndexMap::new(),
            body_patterns: Vec::new(),
            cookies: IndexMap::new(),
            basic_credentials: None,
        }
    }

    pub(crate) fn all_requests() -> RequestPatternBuilder {
        RequestPatternBuilder {
            method: RequestMethod::ANY,
            url_pattern: UrlPattern::any(),
            headers: IndexMap::new(),
            query_params: IndexMap::new(),
            body_patterns: Vec::new(),
            cookies: IndexMap::new(),
            basic_credentials: None,
        }
    }

    pub fn with_url<S>(mut self, url: S) -> RequestPatternBuilder
        where S: Into<String>
    {
        self.url_pattern = crate::url_equal_to(url);
        self
    }

    pub fn with_header<S, P>(mut self, key: S, header_pattern: P) -> RequestPatternBuilder
        where S: Into<String>,
              P: StringValuePattern,
    {
        self.headers.insert(key.into(), header_pattern.into());
        self
    }

    pub fn without_header<S>(mut self, key: S) -> RequestPatternBuilder
        where S: Into<String>,
    {
        self.headers.insert(key.into(), AbsentPattern::new().into());
        self
    }

    pub fn with_query_param<K, P>(mut self, key: K, query_param_pattern: P) -> RequestPatternBuilder
        where K: Into<String>,
              P: StringValuePattern,
    {
        self.query_params.insert(key.into(), query_param_pattern.into());
        self
    }

    pub fn with_cookie<S, P>(mut self, name: S, cookie_value_pattern: P) -> RequestPatternBuilder
        where S: Into<String>,
              P: StringValuePattern,
    {
        self.cookies.insert(name.into(), cookie_value_pattern.into());
        self
    }

    pub fn with_basic_auth(mut self, basic_credentials: BasicCredentials) -> RequestPatternBuilder {
        self.basic_credentials = Some(basic_credentials);
        self
    }

    pub fn with_request_body<P>(mut self, body_pattern: P) -> RequestPatternBuilder
        where P: Into<ContentPattern>,
    {
        self.body_patterns.push(body_pattern.into());
        self
    }

//    public RequestPatternBuilder withRequestBodyPart(MultipartValuePattern multiPattern) {
//        if (multiPattern != null) {
//            multiparts.add(multiPattern);
//        }
//        return this;
//    }
//
//    public RequestPatternBuilder withAnyRequestBodyPart(MultipartValuePatternBuilder multiPatternBuilder) {
//        return withRequestBodyPart(multiPatternBuilder.matchingType(MultipartValuePattern.MatchingType.ANY).build());
//    }
//
//    public RequestPatternBuilder withAllRequestBodyParts(MultipartValuePatternBuilder multiPatternBuilder) {
//        return withRequestBodyPart(multiPatternBuilder.matchingType(MultipartValuePattern.MatchingType.ALL).build());
//    }
//
//    public RequestPatternBuilder andMatching(ValueMatcher<Request> customMatcher) {
//        this.customMatcher = customMatcher;
//        return this;
//    }
//
//    public RequestPatternBuilder andMatching(String customRequestMatcherName) {
//        return andMatching(customRequestMatcherName, Parameters.empty());
//    }
//
//    public RequestPatternBuilder andMatching(String customRequestMatcherName, Parameters parameters) {
//        this.customMatcherDefinition = new CustomMatcherDefinition(customRequestMatcherName, parameters);
//        return this;
//    }

    pub fn build(self) -> RequestPattern {
        RequestPattern {
            method: self.method,
            url_pattern: Some(self.url_pattern),
            query_params: self.query_params,
            headers: self.headers,
            cookies: self.cookies,
            basic_auth_credentials: self.basic_credentials,
            body_patterns: self.body_patterns,
        }
    }
}
