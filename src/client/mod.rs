use reqwest::{Method, RequestBuilder, Response, StatusCode};
use reqwest::header::HeaderValue;
use serde::Serialize;
use uuid::Uuid;

pub use builder::*;
pub use credentials::BasicCredentials;

use crate::client::builder::MappingBuilder;
use crate::global::GlobalSettings;
use crate::http::{Result, RequestMethod, Body};
use crate::matching::*;
use crate::model::{GetGlobalSettingsResult, ListStubMappingsResult};
use crate::security::ClientAuthenticator;
use crate::stubbing::StubMapping;

pub(crate) mod builder;
mod credentials;

pub struct WireMock {
    client: reqwest::Client,
    scheme: String,
    host: String,
    port: u16,
    url_path_prefix: String,
    host_header: Option<HeaderValue>,
    authenticator: Box<dyn ClientAuthenticator>,
}

impl Default for WireMock {
    fn default() -> WireMock {
        WireMockBuilder::new().build()
    }
}

impl From<WireMockBuilder> for WireMock {
    fn from(builder: WireMockBuilder) -> WireMock {
        builder.build()
    }
}

impl WireMock {
    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

//    router.add(POST, "/mappings/save", SaveMappingsTask.class);
//    router.add(POST, "/mappings/reset", ResetToDefaultMappingsTask.class);
//    router.add(GET,  "/mappings/{id}", GetStubMappingTask.class);
//    router.add(PUT,  "/mappings/{id}", EditStubMappingTask.class);
//    router.add(DELETE, "/mappings/{id}", RemoveStubMappingTask.class);
//    router.add(POST, "/mappings/find-by-metadata", FindStubMappingsByMetadataTask.class);
//    router.add(POST, "/mappings/remove-by-metadata", RemoveStubMappingsByMetadataTask.class);
//    router.add(POST, "/mappings/import", ImportStubMappingsTask.class);
//
//    router.add(GET, "/files", GetAllStubFilesTask.class);
//    router.add(PUT, "/files/{filename}", EditStubFileTask.class);
//    router.add(DELETE, "/files/{filename}", DeleteStubFileTask.class);
//
//    router.add(GET, "/scenarios", GetAllScenariosTask.class);
//    router.add(POST, "/scenarios/reset", ResetScenariosTask.class);
//
//    router.add(GET,  "/requests", GetAllRequestsTask.class);
//    router.add(DELETE,  "/requests", ResetRequestsTask.class);
//    router.add(POST, "/requests/reset", OldResetRequestsTask.class);  // Deprecated
//    router.add(POST, "/requests/count", GetRequestCountTask.class);
//    router.add(POST, "/requests/find", FindRequestsTask.class);
//    router.add(GET,  "/requests/unmatched", FindUnmatchedRequestsTask.class);
//    router.add(GET,  "/requests/unmatched/near-misses", FindNearMissesForUnmatchedTask.class);
//    router.add(GET,  "/requests/{id}", GetServedStubTask.class);
//
//    router.add(POST, "/recordings/snapshot", SnapshotTask.class);
//    router.add(POST, "/recordings/start", StartRecordingTask.class);
//    router.add(POST, "/recordings/stop", StopRecordingTask.class);
//    router.add(GET,  "/recordings/status", GetRecordingStatusTask.class);
//    router.add(GET,  "/recorder", GetRecordingsIndexTask.class);
//
//    router.add(POST, "/near-misses/request", FindNearMissesForRequestTask.class);
//    router.add(POST, "/near-misses/request-pattern", FindNearMissesForRequestPatternTask.class);
//
//    router.add(GET, "/settings", GetGlobalSettingsTask.class);
//    router.add(PUT, "/settings", GlobalSettingsUpdateTask.class);
//    router.add(POST, "/settings", GlobalSettingsUpdateTask.class);
//    router.add(PATCH, "/settings/extended", PatchExtendedSettingsTask.class);
//
//    router.add(POST, "/shutdown", ShutdownServerTask.class);
//
//    router.add(GET, "/docs/swagger", GetSwaggerSpecTask.class);
//    router.add(GET, "/docs", GetDocIndexTask.class);

    pub fn given_that<S: Into<StubMapping>>(&self, stub_mapping: S) -> Result<StubMapping> {
        let stub_mapping = stub_mapping.into();
        self.add_stub_mapping(&stub_mapping)?;
        Ok(stub_mapping)
    }

    pub fn stub_for<S: Into<StubMapping>>(&self, stub_mapping: S) -> Result<StubMapping> {
        self.given_that(stub_mapping)
    }

    pub fn add_stub_mapping(&self, stub_mapping: &StubMapping) -> Result<()> {
        self.send_json_request(Method::POST, "/mappings", stub_mapping)
            .map(|_| ())
    }

    pub fn edit_stub_mapping(&self, stub_mapping: &StubMapping) -> Result<()> {
        self.send_json_request(Method::PUT, &format!("/mappings/{}", stub_mapping.id), stub_mapping)
            .map(|_| ())
    }

    pub fn remove_stub_mapping(&self, id: &Uuid) -> Result<bool> {
        self.send_empty_request(Method::DELETE, &format!("/mappings/{}", id))
            .map(|_| true)
            .or_else(|error| {
                if let Some(status_code) = error.status() {
                    if status_code == StatusCode::NOT_FOUND {
                        return Ok(false);
                    }
                }

                Err(error)
            })
    }

    pub fn list_all_stub_mappings(&self) -> Result<ListStubMappingsResult> {
        self.send_empty_request(Method::GET, "/")
            .and_then(|mut response| response.json::<ListStubMappingsResult>())
    }

    pub fn reset_all(&self) -> Result<()> {
        self.send_empty_request(Method::POST, "/reset")
            .map(|_| ())
    }

    pub fn reset_requests(&self) -> Result<()> {
        self.send_empty_request(Method::DELETE, "/requests")
            .map(|_| ())
    }

    pub fn reset_scenarios(&self) -> Result<()> {
        self.send_empty_request(Method::POST, "/scenarios/reset")
            .map(|_| ())
    }

    pub fn reset_mappings(&self) -> Result<()> {
        self.send_empty_request(Method::DELETE, "/mappings")
            .map(|_| ())
    }

    pub fn reset_to_default_mappings(&self) -> Result<()> {
        self.send_empty_request(Method::POST, "/mappings/reset")
            .map(|_| ())
    }

    pub fn shutdown_server(&self) -> Result<()> {
        self.send_empty_request(Method::POST, "/shutdown")
            .map(|_| ())
    }

    pub fn update_global_settings(&self, global_settings: &GlobalSettings) -> Result<()> {
        self.send_json_request(Method::POST, "/settings", global_settings)
            .map(|_| ())
    }

    pub fn get_global_settings(&self) -> Result<GetGlobalSettingsResult> {
        self.send_empty_request(Method::GET, "/settings")
            .and_then(|mut response| response.json::<GetGlobalSettingsResult>())
    }


    fn send_empty_request(&self, method: Method, path: &str) -> Result<Response> {
        let request = self.create_request(method, path);

        request.send()
            .and_then(|response| response.error_for_status())
    }

    fn send_json_request<T>(&self, method: Method, path: &str, json: &T) -> Result<Response>
        where T: Serialize + ?Sized
    {
        let request = self.create_request(method, path);

        request.json(json)
            .send()
            .and_then(|response| response.error_for_status())
    }

    fn create_request(&self, method: Method, path: &str) -> RequestBuilder {
        let url = format!("{}://{}:{}{}/__admin{}",
                          self.scheme, self.host, self.port, self.url_path_prefix, path);

        let mut request = self.client.request(method, &url);

        if let Some(host_header) = self.host_header.as_ref() {
            request = request.header(reqwest::header::HOST, host_header);
        };

        for (header_name, header_value) in self.authenticator.generate_auth_headers().iter() {
            request = request.header(header_name, header_value);
        };

        request
    }
}

pub fn equal_to<S>(value: S) -> EqualToPattern
    where S: Into<String>,
{
    EqualToPattern::new(value, None)
}

pub fn binary_equal_to<B>(content: B) -> BinaryEqualToPattern
    where B: Into<Vec<u8>>,
{
    BinaryEqualToPattern::new(content)
}

pub fn equal_to_ignore_case<S>(value: S) -> EqualToPattern
    where S: Into<String>,
{
    EqualToPattern::new(value, Some(true))
}

pub fn equal_to_json<S>(json: S) -> EqualToJsonPattern
    where S: Into<String>,
{
    EqualToJsonPattern::new(json)
}

pub fn matching_json_path<S>(json_path: S) -> MatchesJsonPathPattern
    where S: Into<String>,
{
    MatchesJsonPathPattern::new(json_path)
}

pub fn equal_to_xml<S>(xml: S) -> EqualToXmlPattern
    where S: Into<String>,
{
    EqualToXmlPattern::new(xml)
}

pub fn matching_xpath<S>(value: S) -> MatchesXPathPattern
    where S: Into<String>,
{
    MatchesXPathPattern::new(value)
}

pub fn containing<S>(value: S) -> ContainsPattern
    where S: Into<String>,
{
    ContainsPattern::new(value)
}

pub fn matching<S>(regex: S) -> RegexPattern
    where S: Into<String>,
{
    RegexPattern::new(regex)
}

pub fn not_matching<S>(regex: S) -> NegativeRegexPattern
    where S: Into<String>,
{
    NegativeRegexPattern::new(regex)
}

pub fn absent() -> AbsentPattern {
    AbsentPattern::new()
}

pub fn get<P>(url_pattern: P) -> MappingBuilder
    where P: Into<UrlPattern>,
{
    MappingBuilder::new(RequestMethod::GET, url_pattern.into())
}

pub fn post<P>(url_pattern: P) -> MappingBuilder
    where P: Into<UrlPattern>,
{
    MappingBuilder::new(RequestMethod::POST, url_pattern.into())
}

pub fn put<P>(url_pattern: P) -> MappingBuilder
    where P: Into<UrlPattern>,
{
    MappingBuilder::new(RequestMethod::PUT, url_pattern.into())
}

pub fn delete<P>(url_pattern: P) -> MappingBuilder
    where P: Into<UrlPattern>,
{
    MappingBuilder::new(RequestMethod::DELETE, url_pattern.into())
}

pub fn patch<P>(url_pattern: P) -> MappingBuilder
    where P: Into<UrlPattern>,
{
    MappingBuilder::new(RequestMethod::PATCH, url_pattern.into())
}

pub fn head<P>(url_pattern: P) -> MappingBuilder
    where P: Into<UrlPattern>,
{
    MappingBuilder::new(RequestMethod::HEAD, url_pattern.into())
}

pub fn options<P>(url_pattern: P) -> MappingBuilder
    where P: Into<UrlPattern>,
{
    MappingBuilder::new(RequestMethod::OPTIONS, url_pattern.into())
}

pub fn trace<P>(url_pattern: P) -> MappingBuilder
    where P: Into<UrlPattern>,
{
    MappingBuilder::new(RequestMethod::TRACE, url_pattern.into())
}

pub fn any<P>(url_pattern: P) -> MappingBuilder
    where P: Into<UrlPattern>,
{
    MappingBuilder::new(RequestMethod::ANY, url_pattern.into())
}

pub fn request<M, P>(method: M, url_pattern: P) -> MappingBuilder
    where M: Into<RequestMethod>,
          P: Into<UrlPattern>,
{
    MappingBuilder::new(method.into(), url_pattern.into())
}

//public static MappingBuilder requestMatching(String customRequestMatcherName) {
//return new BasicMappingBuilder(customRequestMatcherName, Parameters.empty());
//}
//
//public static MappingBuilder requestMatching(String customRequestMatcherName, Parameters parameters) {
//return new BasicMappingBuilder(customRequestMatcherName, parameters);
//}
//
//public static MappingBuilder requestMatching(ValueMatcher<Request> requestMatcher) {
//return new BasicMappingBuilder(requestMatcher);
//}

pub fn a_response() -> ResponseDefinitionBuilder {
    ResponseDefinitionBuilder::new()
}

pub fn ok() -> ResponseDefinitionBuilder {
    ResponseDefinitionBuilder::new().with_status(reqwest::StatusCode::OK.as_u16())
}

pub fn ok_with_body<B>(body: B) -> ResponseDefinitionBuilder
    where B: Into<Body>,
{
    ok().with_body(body)
}

pub fn ok_for_content_type<S, B>(content_type: S, body: B) -> ResponseDefinitionBuilder
    where S: Into<String>,
          B: Into<Body>,
{
    ok()
        .with_header(reqwest::header::CONTENT_TYPE.to_string(), content_type.into())
        .with_body(body)
}

pub fn ok_json<B>(body: B) -> ResponseDefinitionBuilder
    where B: Into<Body>,
{
    ok_for_content_type("application/json", body)
}

pub fn ok_xml<B>(body: B) -> ResponseDefinitionBuilder
    where B: Into<Body>,
{
    ok_for_content_type("application/xml", body)
}

pub fn ok_text_xml<B>(body: B) -> ResponseDefinitionBuilder
    where B: Into<Body>,
{
    ok_for_content_type("text/xml", body)
}

pub fn proxy_all_to<S>(url: S) -> MappingBuilder
    where S: Into<String>,
{
    any(any_url()).will_return(a_response().proxied_from(url))
}

pub fn created() -> ResponseDefinitionBuilder {
    ResponseDefinitionBuilder::new().with_status(reqwest::StatusCode::CREATED.as_u16())
}

pub fn no_content() -> ResponseDefinitionBuilder {
    ResponseDefinitionBuilder::new().with_status(reqwest::StatusCode::NO_CONTENT.as_u16())
}

pub fn permanent_redirect<S>(location: S) -> ResponseDefinitionBuilder
    where S: Into<String>,
{
    ResponseDefinitionBuilder::new()
        .with_status(reqwest::StatusCode::PERMANENT_REDIRECT.as_u16())
        .with_header(reqwest::header::LOCATION.to_string(), location)
}

pub fn temporary_redirect<S>(location: S) -> ResponseDefinitionBuilder
    where S: Into<String>,
{
    ResponseDefinitionBuilder::new()
        .with_status(reqwest::StatusCode::TEMPORARY_REDIRECT.as_u16())
        .with_header(reqwest::header::LOCATION.to_string(), location)
}

pub fn see_other<S>(location: S) -> ResponseDefinitionBuilder
    where S: Into<String>,
{
    ResponseDefinitionBuilder::new()
        .with_status(reqwest::StatusCode::SEE_OTHER.as_u16())
        .with_header(reqwest::header::LOCATION.to_string(), location)
}

pub fn bad_request() -> ResponseDefinitionBuilder {
    status(reqwest::StatusCode::BAD_REQUEST.as_u16())
}

pub fn bad_request_entity() -> ResponseDefinitionBuilder {
    unprocessable_entity()
}

pub fn unprocessable_entity() -> ResponseDefinitionBuilder {
    ResponseDefinitionBuilder::new().with_status(reqwest::StatusCode::UNPROCESSABLE_ENTITY.as_u16())
}

pub fn unauthorized() -> ResponseDefinitionBuilder {
    ResponseDefinitionBuilder::new().with_status(reqwest::StatusCode::UNAUTHORIZED.as_u16())
}

pub fn forbidden() -> ResponseDefinitionBuilder {
    ResponseDefinitionBuilder::new().with_status(reqwest::StatusCode::FORBIDDEN.as_u16())
}

pub fn not_found() -> ResponseDefinitionBuilder {
    ResponseDefinitionBuilder::new().with_status(reqwest::StatusCode::NOT_FOUND.as_u16())
}

pub fn server_error() -> ResponseDefinitionBuilder {
    ResponseDefinitionBuilder::new().with_status(reqwest::StatusCode::INTERNAL_SERVER_ERROR.as_u16())
}

pub fn service_unavailable() -> ResponseDefinitionBuilder {
    ResponseDefinitionBuilder::new().with_status(reqwest::StatusCode::SERVICE_UNAVAILABLE.as_u16())
}

pub fn status(status: u16) -> ResponseDefinitionBuilder {
    ResponseDefinitionBuilder::new().with_status(status)
}

pub fn url_equal_to<S>(url: S) -> UrlPattern
    where S: Into<String>
{
    UrlPattern::Url(url.into())
}

pub fn url_matching<S>(url_regex: S) -> UrlPattern
    where S: Into<String>
{
    UrlPattern::UrlPattern(url_regex.into())
}

pub fn url_path_equal_to<S>(url_path: S) -> UrlPattern
    where S: Into<String>
{
    UrlPattern::UrlPath(url_path.into())
}

pub fn url_path_matching<S>(url_path_regex: S) -> UrlPattern
    where S: Into<String>
{
    UrlPattern::UrlPathPattern(url_path_regex.into())
}

pub fn any_url() -> UrlPattern {
    UrlPattern::any()
}

pub fn get_requested_for<P>(url_pattern: P) -> RequestPatternBuilder
    where P: Into<UrlPattern>,
{
    RequestPatternBuilder::new(RequestMethod::GET, url_pattern.into())
}

pub fn post_requested_for<P>(url_pattern: P) -> RequestPatternBuilder
    where P: Into<UrlPattern>,
{
    RequestPatternBuilder::new(RequestMethod::POST, url_pattern.into())
}

pub fn put_requested_for<P>(url_pattern: P) -> RequestPatternBuilder
    where P: Into<UrlPattern>,
{
    RequestPatternBuilder::new(RequestMethod::PUT, url_pattern.into())
}

pub fn delete_requested_for<P>(url_pattern: P) -> RequestPatternBuilder
    where P: Into<UrlPattern>,
{
    RequestPatternBuilder::new(RequestMethod::DELETE, url_pattern.into())
}

pub fn patch_requested_for<P>(url_pattern: P) -> RequestPatternBuilder
    where P: Into<UrlPattern>,
{
    RequestPatternBuilder::new(RequestMethod::PATCH, url_pattern.into())
}

pub fn head_requested_for<P>(url_pattern: P) -> RequestPatternBuilder
    where P: Into<UrlPattern>,
{
    RequestPatternBuilder::new(RequestMethod::HEAD, url_pattern.into())
}

pub fn options_requested_for<P>(url_pattern: P) -> RequestPatternBuilder
    where P: Into<UrlPattern>,
{
    RequestPatternBuilder::new(RequestMethod::OPTIONS, url_pattern.into())
}

pub fn trace_requested_for<P>(url_pattern: P) -> RequestPatternBuilder
    where P: Into<UrlPattern>,
{
    RequestPatternBuilder::new(RequestMethod::TRACE, url_pattern.into())
}

pub fn any_requested_for<P>(url_pattern: P) -> RequestPatternBuilder
    where P: Into<UrlPattern>,
{
    RequestPatternBuilder::new(RequestMethod::ANY, url_pattern.into())
}

#[cfg(test)]
mod compile_only_dsl_examples {
    use super::*;
    use crate::stubbing::Scenario;
    use crate::http::Fault;

    #[test]
    #[ignore = "this is a test that only checks if the code compiles"]
    fn exact_url_only() {
        stub_for(get(url_equal_to("/some/thing"))
            .will_return(a_response()
                .with_header("Content-Type", "text/plain")
                .with_body("Hello world!")));
    }

    #[test]
    #[ignore = "this is a test that only checks if the code compiles"]
    fn url_regex_match() {
        stub_for(put(url_matching("/thing/matching/[0-9]+"))
            .will_return(a_response().with_status(200)));
    }

    #[test]
    #[ignore = "this is a test that only checks if the code compiles"]
    fn header_matching() {
        stub_for(post(url_equal_to("/with/headers"))
            .with_header("Content-Type", equal_to("text/xml"))
            .with_header("Accept", matching("text/.*"))
            .with_header("etag", not_matching("abcd.*"))
            .with_header("etag", containing("2134"))
            .will_return(a_response().with_status(200)));
    }

    #[test]
    #[ignore = "this is a test that only checks if the code compiles"]
    fn body_matching() {
        stub_for(post(url_equal_to("/with/body"))
            .with_request_body(matching("<status>OK</status>"))
            .with_request_body(not_matching("<status>ERROR</status>"))
            .will_return(a_response().with_status(200)));
    }

    #[test]
    #[ignore = "this is a test that only checks if the code compiles"]
    fn binary_body_matching_byte_array() {
        stub_for(post(url_equal_to("/with/body"))
            .with_request_body(binary_equal_to(vec![1, 2, 3]))
            .will_return(ok()));
    }

    #[test]
    #[ignore = "this is a test that only checks if the code compiles"]
    fn binary_body_matching_base64() {
        stub_for(post(url_equal_to("/with/body"))
            .with_request_body(binary_equal_to("AQID"))
            .will_return(ok()));
    }

//    #[test]
//    #[ignore = "this is a test that only checks if the code compiles"]
//    fn multipart_body_matching_base64() {
//        stub_for(post(url_equal_to("/with/multipart"))
//            .withMultipartRequestBody(aMultipart()
//                .with_body(binary_equal_to("Content")))
//            .will_return(ok()));
//    }

    #[test]
    #[ignore = "this is a test that only checks if the code compiles"]
    fn priorities() {
        //Catch-all case
        stub_for(get(url_matching("/api/.*")).at_priority(5)
            .will_return(a_response().with_status(401)));

        //Specific case
        stub_for(get(url_equal_to("/api/specific-resource")).at_priority(1) //1 is highest
            .will_return(a_response()
                .with_status(200)
                .with_body("Resource state")));
    }

    #[test]
    #[ignore = "this is a test that only checks if the code compiles"]
    fn response_headers() {
        stub_for(get(url_equal_to("/whatever"))
            .will_return(a_response()
                .with_status(200)
                .with_header("Content-Type", "application/json")
                .with_header("Etag", "b13894794wb")));
    }

    #[test]
    #[ignore = "this is a test that only checks if the code compiles"]
    fn body_file() {
        stub_for(get(url_equal_to("/body-file"))
            .will_return(a_response()
                .with_body_file("path/to/myfile.xml")));
    }

    #[test]
    #[ignore = "this is a test that only checks if the code compiles"]
    fn binary_body() {
        stub_for(get(url_equal_to("/binary-body"))
            .will_return(a_response()
                .with_body(vec![1, 2, 3, 4])));
    }

//    #[test]
//    #[ignore = "this is a test that only checks if the code compiles"]
//    fn finding_requests() {
//        let requests: Vec<LoggedRequest> = findAll(put_requested_for(url_matching("/api/.*")));
//    }

    #[test]
    #[ignore = "this is a test that only checks if the code compiles"]
    fn proxying() {
        stub_for(get(url_matching("/other/service/.*"))
            .will_return(a_response().proxied_from("http://otherhost.com/approot")));
    }

    #[test]
    #[ignore = "this is a test that only checks if the code compiles"]
    fn proxy_intercept() {
        // Low priority catch-all proxies to otherhost.com by default
        stub_for(get(url_matching(".*")).at_priority(10)
            .will_return(a_response().proxied_from("http://otherhost.com")));

        // High priority stub will send a Service Unavailable response
        // if the specified URL is requested
        stub_for(get(url_equal_to("/api/override/123")).at_priority(1)
            .will_return(a_response().with_status(503)));
    }

    #[test]
    #[ignore = "this is a test that only checks if the code compiles"]
    fn to_do_list_scenario() {
        stub_for(get(url_equal_to("/todo/items")).in_scenario("To do list")
            .when_scenario_state_is(Scenario::STARTED)
            .will_return(a_response()
                .with_body("<items>".to_string() +
                    "   <item>Buy milk</item>" +
                    "</items>")));

        stub_for(post(url_equal_to("/todo/items")).in_scenario("To do list")
            .when_scenario_state_is(Scenario::STARTED)
            .with_request_body(containing("Cancel newspaper subscription"))
            .will_return(a_response().with_status(201))
            .will_set_state_to("Cancel newspaper item added"));

        stub_for(get(url_equal_to("/todo/items")).in_scenario("To do list")
            .when_scenario_state_is("Cancel newspaper item added")
            .will_return(a_response()
                .with_body("<items>".to_string() +
                    "   <item>Buy milk</item>" +
                    "   <item>Cancel newspaper subscription</item>" +
                    "</items>")));
    }

    #[test]
    #[ignore = "this is a test that only checks if the code compiles"]
    fn delay() {
        stub_for(get(url_equal_to("/delayed")).will_return(
            a_response()
                .with_status(200)
                .with_fixed_delay(2000)));
    }

    #[test]
    #[ignore = "this is a test that only checks if the code compiles"]
    fn fault() {
        stub_for(get(url_equal_to("/fault"))
            .will_return(a_response().with_fault(Fault::MALFORMED_RESPONSE_CHUNK)));
    }

    #[test]
    #[ignore = "this is a test that only checks if the code compiles"]
    fn xpath() {
        stub_for(put(url_equal_to("/xpath"))
            .with_request_body(matching_xpath("/todo-list[count(todo-item) = 3]"))
            .will_return(a_response().with_status(200)));
    }

    #[test]
    #[ignore = "this is a test that only checks if the code compiles"]
    fn xpath_with_namespaces() {
        stub_for(put(url_equal_to("/namespaced/xpath"))
            .with_request_body(matching_xpath("/stuff:outer/stuff:inner[.=111]")
                .with_xpath_namespace("stuff", "http://foo.com"))
            .will_return(a_response().with_status(200)));
    }

//    #[test]
//    #[ignore = "this is a test that only checks if the code compiles"]
//    fn advanced_xpath_matching() {
//        stub_for(put(url_equal_to("/xpath"))
//            .with_request_body(matching_xpath("//todo-item/text()", containing("wash")))
//            .will_return(a_response().with_status(200)));
//    }

//    #[test]
//    #[ignore = "this is a test that only checks if the code compiles"]
//    fn advanced_jsonpath_matching() {
//        stub_for(put(url_equal_to("/jsonpath"))
//            .with_request_body(matching_json_path("$..todoItem", containing("wash")))
//            .will_return(a_response().with_status(200)));
//    }

    #[test]
    #[ignore = "this is a test that only checks if the code compiles"]
    fn transformer() {
        stub_for(get(url_equal_to("/transform")).will_return(
            a_response()
                .with_transformer("body-transformer")));
    }

    #[test]
    #[ignore = "this is a test that only checks if the code compiles"]
    fn transformer_parameters() {
        let mut inner_param = serde_json::Map::new();
        inner_param.insert("thing".to_string(), "value".into());

        stub_for(get(url_equal_to("/transform")).will_return(
            a_response()
                .with_transformer_parameter("newValue", 66)
                .with_transformer_parameter("inner", inner_param)));
    }

//    #[test]
//    #[ignore = "this is a test that only checks if the code compiles"]
//    fn transformer_with_parameters() {
//        stub_for(get(url_equal_to("/transform")).will_return(
//            a_response()
//                .with_transformer("body-transformer", "newValue", 66)));
//    }

    #[test]
    #[ignore = "this is a test that only checks if the code compiles"]
    fn abbreviated_dsl() {
        stub_for(get("/some/thing").will_return(a_response().with_status(200)));

        stub_for(delete("/fine").will_return(ok()));
        stub_for(get("/json").will_return(ok_json("{ \"message\": \"Hello\" }")));
        stub_for(get("/xml").will_return(ok_xml("<hello />")));     // application/xml
        stub_for(get("/xml").will_return(ok_text_xml("<hello />"))); // text/xml
        stub_for(post("/things").will_return(no_content()));

        stub_for(post("/temp-redirect").will_return(temporary_redirect("/new/place")));
        stub_for(post("/perm-redirect").will_return(permanent_redirect("/new/place")));
        stub_for(post("/see-other").will_return(see_other("/new/place")));

        stub_for(post("/sorry-no").will_return(unauthorized()));
        stub_for(post("/still-no").will_return(forbidden()));

        stub_for(put("/dodgy").will_return(bad_request()));
        stub_for(put("/dodgy-body").will_return(bad_request_entity()));
        stub_for(put("/nothing-to-see-here").will_return(not_found()));

        stub_for(put("/status-only").will_return(status(418)));

        stub_for(get("/dead-server").will_return(service_unavailable()));
        stub_for(put("/error").will_return(server_error()));

        stub_for(proxy_all_to("http://my.example.com"));
    }


    fn stub_for<S: Into<StubMapping>>(stub_mapping: S) {
        let wire_mock = WireMock::default();
        wire_mock.stub_for(stub_mapping).unwrap();
    }
}
