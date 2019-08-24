use std::borrow::Cow;

use http::{Method, StatusCode};
use http::HeaderValue;
use reqwest::{RequestBuilder, Response};
use serde::Serialize;
use uuid::Uuid;

pub use builder::*;
pub use credentials::BasicCredentials;
pub use dsl::*;

use crate::client::builder::MappingBuilder;
use crate::global::GlobalSettings;
use crate::http::{Error, Result};
use crate::matching::{RequestPattern, StringValuePattern, ContentPattern};
use crate::model::{GetGlobalSettingsResult, GetScenariosResult, GetServeEventsResult, ListStubMappingsResult, SingleServedStubResult, SingleStubMappingResult};
use crate::security::ClientAuthenticator;
use crate::stubbing::{Scenario, ServeEvent, StubMapping};
use crate::verification::{FindNearMissesResult, FindRequestsResult, JournalBasedResult, LoggedRequest, NearMiss, VerificationResult};

pub(crate) mod builder;
mod credentials;
mod dsl;

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

//    router.add(POST, "/mappings/import", ImportStubMappingsTask.class);

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
            .or_else(|error| map_not_found_error_to(error, false))
    }

    pub fn list_all_stub_mappings(&self) -> Result<ListStubMappingsResult> {
        self.send_empty_request(Method::GET, "/")
            .and_then(|mut response| response.json::<ListStubMappingsResult>())
    }

    pub fn list_stub_mappings(&self) -> Result<Vec<StubMapping>> {
        self.list_all_stub_mappings()
            .map(ListStubMappingsResult::into)
    }

    pub fn get_stub_mapping(&self, id: &Uuid) -> Result<Option<StubMapping>> {
        self.send_empty_request(Method::GET, &format!("/mappings/{}", id))
            .and_then(|mut response| response.json::<SingleStubMappingResult>())
            .map(|result| Some(result.into()))
            .or_else(map_not_found_error_to_none)
    }

    pub fn save_mappings(&self) -> Result<()> {
        self.send_empty_request(Method::POST, "/mappings/save")
            .map(|_| ())
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

    pub fn get_serve_events(&self) -> Result<Vec<ServeEvent>> {
        self.send_empty_request(Method::GET, "/requests")
            .and_then(|mut response| response.json::<GetServeEventsResult>())
            .map(GetServeEventsResult::into)
    }

    pub fn get_served_stub(&self, id: &Uuid) -> Result<Option<ServeEvent>> {
        self.send_empty_request(Method::GET, &format!("/requests/{}", id))
            .and_then(|mut response| response.json::<SingleServedStubResult>())
            .map(|result| Some(result.into()))
            .or_else(map_not_found_error_to_none)
    }

    pub fn count_requests_matching<'a, P>(&self, request_pattern: P) -> Result<VerificationResult>
        where P: Into<Cow<'a, RequestPattern>>,
    {
        self.send_json_request(Method::POST, "/requests/count", &request_pattern.into())
            .and_then(|mut response| response.json::<VerificationResult>())
    }

    pub fn count<'a, P>(&self, request_pattern: P) -> Result<u32>
        where P: Into<Cow<'a, RequestPattern>>,
    {
        self.count_requests_matching(request_pattern)
            .and_then(|verification_result| {
                verification_result.assert_request_journal_enabled();
                Ok(verification_result.count().unwrap())
            })
    }

    pub fn find_requests_matching<'a, P>(&self, request_pattern: P) -> Result<FindRequestsResult>
        where P: Into<Cow<'a, RequestPattern>>,
    {
        self.send_json_request(Method::POST, "/requests/find", &request_pattern.into())
            .and_then(|mut response| response.json::<FindRequestsResult>())
    }

    pub fn find<'a, P>(&self, request_pattern: P) -> Result<Vec<LoggedRequest>>
        where P: Into<Cow<'a, RequestPattern>>,
    {
        self.find_requests_matching(request_pattern)
            .and_then(|find_requests_result| {
                find_requests_result.assert_request_journal_enabled();
                Ok(find_requests_result.into())
            })
    }

    pub fn find_unmatched_requests(&self) -> Result<FindRequestsResult> {
        self.send_empty_request(Method::GET, "/requests/unmatched")
            .and_then(|mut response| response.json::<FindRequestsResult>())
    }

    pub fn find_unmatched(&self) -> Result<Vec<LoggedRequest>> {
        self.find_unmatched_requests()
            .and_then(|find_requests_result| {
                find_requests_result.assert_request_journal_enabled();
                Ok(find_requests_result.into())
            })
    }

    pub fn find_top_near_misses_for_request(&self, logged_request: &LoggedRequest) -> Result<FindNearMissesResult> {
        self.send_json_request(Method::POST, "/near-misses/request", logged_request)
            .and_then(|mut response| response.json::<FindNearMissesResult>())
    }

    pub fn find_near_misses_for_request(&self, logged_request: &LoggedRequest) -> Result<Vec<NearMiss>> {
        self.find_top_near_misses_for_request(logged_request)
            .map(FindNearMissesResult::into)
    }

    pub fn find_top_near_misses_for<'a, P>(&self, request_pattern: P) -> Result<FindNearMissesResult>
        where P: Into<Cow<'a, RequestPattern>>,
    {
        self.send_json_request(Method::POST, "/near-misses/request-pattern", &request_pattern.into())
            .and_then(|mut response| response.json::<FindNearMissesResult>())
    }

    pub fn find_near_misses_for<'a, P>(&self, request_pattern: P) -> Result<Vec<NearMiss>>
        where P: Into<Cow<'a, RequestPattern>>,
    {
        self.find_top_near_misses_for(request_pattern)
            .map(FindNearMissesResult::into)
    }

    pub fn find_top_near_misses_for_unmatched_requests(&self) -> Result<FindNearMissesResult> {
        self.send_empty_request(Method::GET, "/requests/unmatched/near-misses")
            .and_then(|mut response| response.json::<FindNearMissesResult>())
    }

    pub fn find_near_misses_for_unmatched_requests(&self) -> Result<Vec<NearMiss>> {
        self.find_top_near_misses_for_unmatched_requests()
            .map(FindNearMissesResult::into)
    }

    pub fn get_all_scenarios(&self) -> Result<GetScenariosResult> {
        self.send_empty_request(Method::GET, "/scenarios")
            .and_then(|mut response| response.json::<GetScenariosResult>())
    }

    pub fn get_scenarios(&self) -> Result<Vec<Scenario>> {
        self.get_all_scenarios()
            .map(GetScenariosResult::into)
    }

    pub fn shutdown_server(&self) -> Result<()> {
        self.send_empty_request(Method::POST, "/shutdown")
            .map(|_| ())
    }

    pub fn find_all_stubs_by_metadata<P>(&self, pattern: P) -> Result<ListStubMappingsResult>
        where P: StringValuePattern + Sized,
    {
        let content_pattern: ContentPattern = pattern.into();
        self.send_json_request(Method::POST, "/mappings/find-by-metadata", &content_pattern)
            .and_then(|mut response| response.json::<ListStubMappingsResult>())
    }

    pub fn find_stubs_by_metadata<P>(&self, pattern: P) -> Result<Vec<StubMapping>>
        where P: StringValuePattern + Sized,
    {
        self.find_all_stubs_by_metadata(pattern)
            .map(ListStubMappingsResult::into)
    }

    pub fn remove_stubs_by_metadata<P>(&self, pattern: P) -> Result<()>
        where P: StringValuePattern + Sized,
    {
        let content_pattern: ContentPattern = pattern.into();
        self.send_json_request(Method::POST, "/mappings/remove-by-metadata", &content_pattern)
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
            request = request.header(http::header::HOST, host_header);
        };

        for (header_name, header_value) in self.authenticator.generate_auth_headers().iter() {
            request = request.header(header_name, header_value);
        };

        request
    }
}

fn map_not_found_error_to_none<T>(error: Error) -> Result<Option<T>> {
    map_not_found_error_to(error, None)
}

fn map_not_found_error_to<T>(error: Error, value_on_not_found_error: T) -> Result<T> {
    if let Some(status_code) = error.status() {
        if status_code == StatusCode::NOT_FOUND {
            return Ok(value_on_not_found_error);
        }
    }

    Err(error)
}

#[cfg(test)]
mod compile_only_dsl_examples {
    use crate::http::Fault;
    use crate::stubbing::Scenario;

    use super::*;

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

    #[test]
    #[ignore = "this is a test that only checks if the code compiles"]
    fn count_requests_matching_with_borrowed_request_pattern() {
        let request = get_requested_for(any_url()).build();
        WireMock::default().count_requests_matching(&request).unwrap();
    }

    #[test]
    #[ignore = "this is a test that only checks if the code compiles"]
    fn count_requests_matching_with_owned_request_pattern() {
        let request = get_requested_for(any_url()).build();
        WireMock::default().count_requests_matching(request).unwrap();
    }

    #[test]
    #[ignore = "this is a test that only checks if the code compiles"]
    fn count_requests_matching_with_owned_request_pattern_builder() {
        WireMock::default().count_requests_matching(get_requested_for(any_url())).unwrap();
    }

    #[test]
    #[ignore = "this is a test that only checks if the code compiles"]
    fn find_requests_matching_with_borrowed_request_pattern() {
        let request = get_requested_for(any_url()).build();
        WireMock::default().find_requests_matching(&request).unwrap();
    }

    #[test]
    #[ignore = "this is a test that only checks if the code compiles"]
    fn find_requests_matching_with_owned_request_pattern() {
        let request = get_requested_for(any_url()).build();
        WireMock::default().find_requests_matching(request).unwrap();
    }

    #[test]
    #[ignore = "this is a test that only checks if the code compiles"]
    fn find_requests_matching_with_owned_request_pattern_builder() {
        WireMock::default().find_requests_matching(get_requested_for(any_url())).unwrap();
    }

    fn stub_for<S: Into<StubMapping>>(stub_mapping: S) {
        let wire_mock = WireMock::default();
        wire_mock.stub_for(stub_mapping).unwrap();
    }
}
