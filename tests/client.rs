use uuid::Uuid;

use wiremock_client::{a_response, any, any_url, containing, equal_to, get, get_requested_for, matching_json_path, no_content, ok, ok_with_body, post, post_requested_for, put, url_equal_to, url_path_equal_to, WireMock, WireMockBuilder, less_than};
use wiremock_client::common::metadata;
use wiremock_client::global::GlobalSettingsBuilder;
use wiremock_client::http::DelayDistribution;
use wiremock_client::stubbing::Scenario;
use wiremock_client::stubbing::stub_import;
use wiremock_client::verification::JournalBasedResult;

macro_rules! string_json_map {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(string_json_map!(@single $rest)),*]));

    ($($key:expr => $value:expr,)+) => { string_json_map!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            let _cap = string_json_map!(@count $($key),*);
            let mut _map = indexmap::IndexMap::with_capacity(_cap);
            $(
                let _ = _map.insert(String::from($key), serde_json::Value::from($value));
            )*
            _map
        }
    };
}

#[test]
fn stub_for_get_url_equal_to() {
    let wire_mock = create_wire_mock();

    let stub_mapping = wire_mock.stub_for(get(url_equal_to("/some/thing"))).unwrap();

    let stub_mapping_removed = wire_mock.remove_stub_mapping(&stub_mapping.id()).unwrap();
    assert_eq!(stub_mapping_removed, true);
}

#[test]
fn stub_for_get_str() {
    let wire_mock = create_wire_mock();

    let stub_mapping = wire_mock.stub_for(get("/some/thing")).unwrap();

    let stub_mapping_removed = wire_mock.remove_stub_mapping(&stub_mapping.id()).unwrap();
    assert_eq!(stub_mapping_removed, true);
}

#[test]
fn stub_for_get_string() {
    let wire_mock = create_wire_mock();

    let stub_mapping = wire_mock.stub_for(get("/some/thing".to_string())).unwrap();

    let stub_mapping_removed = wire_mock.remove_stub_mapping(&stub_mapping.id()).unwrap();
    assert_eq!(stub_mapping_removed, true);
}

#[test]
fn stub_with_single_header_value_response() {
    let wire_mock = create_wire_mock();

    let stub_mapping = wire_mock.stub_for(get("/some/thing".to_string())
        .will_return(a_response()
            .with_header(http::header::SET_COOKIE, "single-value")))
        .unwrap();
    print_json_value(&wire_mock.get_stub_mapping(stub_mapping.id()).unwrap());

    let stub_mapping_removed = wire_mock.remove_stub_mapping(&stub_mapping.id()).unwrap();
    assert_eq!(stub_mapping_removed, true);
}

#[test]
fn stub_with_multi_header_value_response() {
    let wire_mock = create_wire_mock();

    let stub_mapping = wire_mock.stub_for(get("/some/thing".to_string())
        .will_return(a_response()
            .with_header(http::header::SET_COOKIE, "value1")
            .with_header(http::header::SET_COOKIE, "value2")))
        .unwrap();
    print_json_value(&wire_mock.get_stub_mapping(stub_mapping.id()).unwrap());

    let stub_mapping_removed = wire_mock.remove_stub_mapping(&stub_mapping.id()).unwrap();
    assert_eq!(stub_mapping_removed, true);
}

#[test]
fn add_and_remove_stub_mapping() {
    let wire_mock = create_wire_mock();

    let stub_mapping = wire_mock.stub_for(get(url_equal_to("test"))
        .at_priority(5)
        .with_name("Test: add_stub_mapping")
        .with_request_body(containing("add_stub_mapping"))
        .will_return(ok()
            .with_status_message("OK")
            .with_body("Hello world!"))).unwrap();

    let stub_mapping_removed = wire_mock.remove_stub_mapping(&stub_mapping.id()).unwrap();
    assert_eq!(stub_mapping_removed, true);
}

#[test]
fn add_edit_and_remove_stub_mapping() {
    let wire_mock = create_wire_mock();

    let stub_mapping = wire_mock.stub_for(get(url_equal_to("test"))
        .at_priority(5)
        .with_name("Test: edit_stub_mapping")
        .with_request_body(containing("edit_stub_mapping"))
        .will_return(ok()
            .with_status_message("OK")
            .with_body("Hello world!"))).unwrap();

    let mut edited_stub_mapping = stub_mapping;
    edited_stub_mapping.set_priority(4);
    edited_stub_mapping.set_name("Test: edit_stub_mapping (edited)");
    edited_stub_mapping.set_request(get_requested_for(url_path_equal_to("test"))
        .with_request_body(equal_to("edit_stub_mapping")
            .with_case_insensitive(false)));
    edited_stub_mapping.set_response(no_content()
        .with_status_message("No content"));

    wire_mock.edit_stub_mapping(&edited_stub_mapping).unwrap();

    let stub_mapping_removed = wire_mock.remove_stub_mapping(&edited_stub_mapping.id()).unwrap();
    assert_eq!(stub_mapping_removed, true);
}

#[test]
fn remove_non_existent_stub_mapping() {
    let wire_mock = create_wire_mock();

    let stub_mapping_removed = wire_mock.remove_stub_mapping(&Uuid::new_v4()).unwrap();
    assert_eq!(stub_mapping_removed, false);
}

#[test]
fn get_stub_mapping() {
    let wire_mock = create_wire_mock();

    let stub_mapping = wire_mock.stub_for(any(any_url())).unwrap();

    let opt_stub_mapping = wire_mock.get_stub_mapping(stub_mapping.id()).unwrap();
    assert_eq!(opt_stub_mapping.unwrap().id(), stub_mapping.id());

    let stub_mapping_removed = wire_mock.remove_stub_mapping(stub_mapping.id()).unwrap();
    assert_eq!(stub_mapping_removed, true);
}

#[test]
fn get_non_existent_stub_mapping() {
    let wire_mock = create_wire_mock();

    let opt_stub_mapping = wire_mock.get_stub_mapping(&Uuid::new_v4()).unwrap();
    assert!(opt_stub_mapping.is_none());
}

#[test]
#[ignore = "random stub mappings (created while other tests run) shouldn't be persisted to disk"]
fn save_mappings() {
    let wire_mock = create_wire_mock();

    wire_mock.save_mappings().unwrap();
}

#[test]
fn list_stub_mappings() {
    let wire_mock = create_wire_mock();
    let stub_mappings = wire_mock.list_all_stub_mappings().unwrap();
    print_json_value(&stub_mappings);
}

#[test]
#[ignore = "resetting WireMock interferes with other tests that require state"]
fn reset_all() {
    let wire_mock = create_wire_mock();
    wire_mock.reset_all().unwrap();
}

#[test]
#[ignore = "resetting WireMock interferes with other tests that require state"]
fn reset_requests() {
    let wire_mock = create_wire_mock();
    wire_mock.reset_requests().unwrap();
}

#[test]
#[ignore = "resetting WireMock interferes with other tests that require state"]
fn reset_scenarios() {
    let wire_mock = create_wire_mock();
    wire_mock.reset_scenarios().unwrap();
}

#[test]
#[ignore = "removing stub mappings from the file system may lead to data loss"]
fn reset_mappings() {
    let wire_mock = create_wire_mock();
    wire_mock.reset_mappings().unwrap();
}

#[test]
#[ignore = "resetting WireMock interferes with other tests that require state"]
fn reset_to_default_mappings() {
    let wire_mock = create_wire_mock();
    wire_mock.reset_to_default_mappings().unwrap();
}

#[test]
fn verify() {
    let wire_mock = create_wire_mock();
    let url = format!("/test/verify?id={}", Uuid::new_v4());

    reqwest::get(&format!("http://localhost:8181{}", &url)).unwrap();

    wire_mock.verify(get_requested_for(url_equal_to(url)));
}

#[test]
#[should_panic]
fn verify_panic() {
    let wire_mock = create_wire_mock();
    let url = format!("/test/verify_panic?id={}", Uuid::new_v4());

    reqwest::get(&format!("http://localhost:8181{}", &url)).unwrap();

    wire_mock.verify(get_requested_for(url_equal_to(&url[0..url.len() - 1])));
}

#[test]
fn verify_arbitrary_request_count() {
    let wire_mock = create_wire_mock();
    let url = format!("/test/verify_arbitrary_request_count?id={}", Uuid::new_v4());

    let absolute_url = format!("http://localhost:8181{}", &url);
    for _ in 0..4 {
        reqwest::get(&absolute_url).unwrap();
    }

    wire_mock.verify_count(4, get_requested_for(url_equal_to(url)));
}

#[test]
#[should_panic]
fn verify_arbitrary_request_count_panic() {
    let wire_mock = create_wire_mock();
    let url = format!("/test/verify_arbitrary_request_count?id={}", Uuid::new_v4());

    let absolute_url = format!("http://localhost:8181{}", &url);
    for _ in 0..4 {
        reqwest::get(&absolute_url).unwrap();
    }

    wire_mock.verify_count(3, get_requested_for(url_equal_to(url)));
}

#[test]
fn verifies_less_than_count_with_less_requests() {
    let wire_mock = create_wire_mock();
    let url = format!("/test/verifies_less_than_count_with_less_requests?id={}", Uuid::new_v4());

    let absolute_url = format!("http://localhost:8181{}", &url);
    for _ in 0..4 {
        reqwest::get(&absolute_url).unwrap();
    }

    wire_mock.verify_count(less_than(5), get_requested_for(url_equal_to(url)));
}

#[test]
fn get_serve_events() {
    let wire_mock = create_wire_mock();
    let serve_events = wire_mock.get_serve_events().unwrap();
    print_json_value(&serve_events);
}

#[test]
fn get_served_stub() {
    let wire_mock = create_wire_mock();

    reqwest::Client::new()
        .post("http://localhost:8181/some/thing")
        .body("Hello")
        .send()
        .unwrap();

    let serve_event = &wire_mock.get_serve_events().unwrap()[0];

    let opt_serve_event = wire_mock.get_served_stub(serve_event.id()).unwrap();
    assert!(opt_serve_event.is_some());

    print_json_value(&opt_serve_event.unwrap());
}

#[test]
fn count_requests_matching() {
    let wire_mock = create_wire_mock();

    reqwest::Client::new()
        .post("http://localhost:8181/some/thing")
        .send()
        .unwrap();

    let result = wire_mock.count_requests_matching(post_requested_for("/some/thing")).unwrap();
    print_json_value(&result);

    result.assert_request_journal_enabled();
    assert!(result.count().unwrap() > 0);
}

#[test]
fn count_no_requests_matching() {
    let wire_mock = create_wire_mock();

    let result = wire_mock.count_requests_matching(get_requested_for(format!("/{}", Uuid::new_v4()))
        .with_header(format!("X-{}", Uuid::new_v4()), equal_to(Uuid::new_v4().to_string()))).unwrap();
    print_json_value(&result);

    result.assert_request_journal_enabled();
    assert_eq!(result.count(), Some(0));
}

#[test]
fn find_requests_matching() {
    let wire_mock = create_wire_mock();

    reqwest::Client::new()
        .post("http://localhost:8181/some/thing")
        .send()
        .unwrap();

    let result = wire_mock.find_requests_matching(post_requested_for("/some/thing")).unwrap();
    print_json_value(&result);

    result.assert_request_journal_enabled();
    assert!(!result.requests().is_empty());
}

#[test]
fn find_no_requests_matching() {
    let wire_mock = create_wire_mock();

    let result = wire_mock.find_requests_matching(get_requested_for(format!("/{}", Uuid::new_v4()))
        .with_header(format!("X-{}", Uuid::new_v4()), equal_to(Uuid::new_v4().to_string()))).unwrap();
    print_json_value(&result);

    result.assert_request_journal_enabled();
    assert!(result.requests().is_empty());
}

#[test]
fn find_near_misses_for_request() {
    let wire_mock = create_wire_mock();

    let url = format!("/test-find-near-misses-for/{}", Uuid::new_v4());

    let stub_mapping = wire_mock.stub_for(get(&url)).unwrap();

    reqwest::Client::new()
        .post(&format!("http://localhost:8181{}", &url))
        .send()
        .unwrap();

    let logged_requests = wire_mock.find_unmatched().unwrap();

    let near_misses = wire_mock.find_near_misses_for_request(&logged_requests[0]).unwrap();
    print_json_value(&near_misses);

    let stub_mapping_removed = wire_mock.remove_stub_mapping(stub_mapping.id()).unwrap();
    assert_eq!(stub_mapping_removed, true);

    assert!(!near_misses.is_empty());
}

#[test]
fn find_near_misses_for() {
    let wire_mock = create_wire_mock();

    let url = format!("/test-find-near-misses-for/{}", Uuid::new_v4());

    let stub_mapping = wire_mock.stub_for(get(&url)).unwrap();

    reqwest::Client::new()
        .post(&format!("http://localhost:8181{}", &url))
        .send()
        .unwrap();

    let near_misses = wire_mock.find_near_misses_for(get_requested_for(url)).unwrap();
    print_json_value(&near_misses);

    let stub_mapping_removed = wire_mock.remove_stub_mapping(stub_mapping.id()).unwrap();
    assert_eq!(stub_mapping_removed, true);

    assert!(!near_misses.is_empty());
}

#[test]
fn find_near_misses_for_unmatched_requests() {
    let wire_mock = create_wire_mock();

    let url = format!("/test-find-near-misses-for-unmatched-requests/{}", Uuid::new_v4());

    let stub_mapping = wire_mock.stub_for(get(&url)).unwrap();

    reqwest::Client::new()
        .post(&format!("http://localhost:8181{}", &url))
        .send()
        .unwrap();

    let near_misses = wire_mock.find_near_misses_for_unmatched_requests().unwrap();
    print_json_value(&near_misses);

    let stub_mapping_removed = wire_mock.remove_stub_mapping(stub_mapping.id()).unwrap();
    assert_eq!(stub_mapping_removed, true);

    assert!(!near_misses.is_empty());
}

#[test]
fn find_unmatched_requests() {
    let wire_mock = create_wire_mock();

    let result = wire_mock.find_unmatched_requests().unwrap();
    print_json_value(&result);

    result.assert_request_journal_enabled();
}

#[test]
fn find_unmatched() {
    let wire_mock = create_wire_mock();

    let logged_requests = wire_mock.find_unmatched().unwrap();
    print_json_value(&logged_requests);
}

#[test]
fn get_scenarios() {
    let wire_mock = create_wire_mock();

    let list_before_add_stub = wire_mock.stub_for(get(url_equal_to("/todo/items"))
        .in_scenario("To do list")
        .when_scenario_state_is(Scenario::STARTED)
        .will_return(a_response()
            .with_body("<items>".to_string() +
                "   <item>Buy milk</item>" +
                "</items>")))
        .unwrap();

    let add_stub = wire_mock.stub_for(post(url_equal_to("/todo/items"))
        .in_scenario("To do list")
        .when_scenario_state_is(Scenario::STARTED)
        .with_request_body(containing("Cancel newspaper subscription"))
        .will_return(a_response().with_status(201))
        .will_set_state_to("Cancel newspaper item added"))
        .unwrap();

    let list_after_add_stub = wire_mock.stub_for(get(url_equal_to("/todo/items"))
        .in_scenario("To do list")
        .when_scenario_state_is("Cancel newspaper item added")
        .will_return(a_response()
            .with_body("<items>".to_string() +
                "   <item>Buy milk</item>" +
                "   <item>Cancel newspaper subscription</item>" +
                "</items>")))
        .unwrap();

    let scenarios = wire_mock.get_scenarios().unwrap();
    print_json_value(&scenarios);

    assert_eq!(wire_mock.remove_stub_mapping(list_before_add_stub.id()).unwrap(), true);
    assert_eq!(wire_mock.remove_stub_mapping(add_stub.id()).unwrap(), true);
    assert_eq!(wire_mock.remove_stub_mapping(list_after_add_stub.id()).unwrap(), true);

    let to_do_list_scenario = scenarios.iter()
        .find(|scenario| scenario.name() == "To do list")
        .unwrap();
    assert_eq!(to_do_list_scenario.possible_states().len(), 2);
    assert!(to_do_list_scenario.possible_states().contains(Scenario::STARTED));
    assert!(to_do_list_scenario.possible_states().contains("Cancel newspaper item added"));
}

#[test]
#[ignore = "updating the global settings my interere with other tests"]
fn update_and_reset_global_settings() {
    let wire_mock = create_wire_mock();

    let default_global_settings = wire_mock.get_global_settings()
        .unwrap();

    let global_settings = GlobalSettingsBuilder::new()
        .fixed_delay(Some(50))
        .delay_distribution(Some(DelayDistribution::Uniform { lower: 50, upper: 60 }))
        .extended(string_json_map! { "e1" => "v1", "e2" => "v2" })
        .build();

    wire_mock.update_global_settings(&global_settings).unwrap();

    wire_mock.update_global_settings(&default_global_settings).unwrap();
}

#[test]
fn get_global_settings() {
    let wire_mock = create_wire_mock();
    let get_global_settings = wire_mock.get_global_settings().unwrap();
    print_json_value(&get_global_settings);
}

#[test]
#[ignore = "shutting down the server lets all subsequent tests fail"]
fn shutdown_server() {
    let wire_mock = create_wire_mock();
    wire_mock.shutdown_server().unwrap();
}

#[test]
pub fn default_stub_import() {
    let wire_mock = create_wire_mock();

    let original_stub = wire_mock.stub_for(get("/one")
        .with_id(Uuid::new_v4())
        .with_metadata(metadata()
            .attr("overwrites_existing_stubs_by_default", ""))
        .will_return(ok_with_body("Original")))
        .unwrap();

    wire_mock.import_stubs(stub_import()
        .stub(get("/one")
            .with_id(original_stub.id().clone())
            .with_metadata(metadata()
                .attr("overwrites_existing_stubs_by_default", ""))
            .will_return(ok_with_body("Updated")))
        .stub(post("/two")
            .with_metadata(metadata()
                .attr("overwrites_existing_stubs_by_default", ""))
            .will_return(ok()))
        .stub(put("/three")
            .with_metadata(metadata()
                .attr("overwrites_existing_stubs_by_default", ""))
            .will_return(ok())))
        .unwrap();

    let json_path = matching_json_path("$..overwrites_existing_stubs_by_default");
    let imported_stubs = wire_mock.find_stubs_by_metadata(json_path).unwrap();
    print_json_value(&imported_stubs);

    assert_eq!(imported_stubs.len(), 3);
    for stub in imported_stubs {
        assert_eq!(wire_mock.remove_stub_mapping(stub.id()).unwrap(), true);

        let request = stub.request();

        if request.url_pattern() == Some(&url_equal_to("/one")) {
            assert_eq!(stub.response().body(), Some(&"Updated".into()));
        }
    }
}

#[test]
pub fn create_and_retrieve_stub_metadata() {
    let wire_mock = create_wire_mock();

    let stub = wire_mock.stub_for(get("/with-metadata")
        .with_id(Uuid::new_v4())
        .with_metadata(metadata()
            .attr("one", 1)
            .attr("two", "2")
            .attr("three", true)
            .attr("four", metadata()
                .attr("five", "55555")
            )
            .list("six", vec![1, 2, 3])
        ))
        .unwrap();

    let retrieved_stub = wire_mock.get_stub_mapping(stub.id())
        .unwrap()
        .unwrap();
    assert_eq!(wire_mock.remove_stub_mapping(stub.id()).unwrap(), true);

    let metadata = retrieved_stub.metadata();
    print_json_value(&metadata);

    assert_eq!(metadata.get_u64("one"), Some(1));
    assert_eq!(metadata.get_str("two"), Some("2"));
    assert_eq!(metadata.get_bool("three"), Some(true));

    let four = metadata.get_metadata("four").unwrap();

    assert_eq!(four.get_str("five"), Some("55555"));

    let six = metadata.get_mapped_array("six", |value| value.as_u64()).unwrap();
    assert_eq!(six[0], Some(1));
}

#[test]
pub fn can_find_stubs_by_metadata() {
    let wire_mock = create_wire_mock();

    let stub1 = wire_mock.stub_for(get("/with-metadata")
        .with_id(Uuid::new_v4())
        .with_metadata(metadata()
            .attr("can_find_stubs_by_metadata-four", metadata()
                .attr("can_find_stubs_by_metadata-five", "55555")
            )
            .list("can_find_stubs_by_metadata-six", vec![1, 2, 3])
        )).unwrap();
    let stub2 = wire_mock.stub_for(get("/without-metadata")).unwrap();

    let json_path = "$..can_find_stubs_by_metadata-four.can_find_stubs_by_metadata-five";
    let stubs = wire_mock.find_stubs_by_metadata(matching_json_path(json_path)).unwrap();
    print_json_value(&stubs);
    assert_eq!(wire_mock.remove_stub_mapping(stub1.id()).unwrap(), true);
    assert_eq!(wire_mock.remove_stub_mapping(stub2.id()).unwrap(), true);

    assert_eq!(stubs.len(), 1);
    let retrieved_stub = &stubs[0];
    assert_eq!(retrieved_stub.id(), stub1.id());
}

#[test]
pub fn can_remove_stubs_by_metadata() {
    let wire_mock = create_wire_mock();

    let stub1 = wire_mock.stub_for(get("/with-metadata")
        .with_id(Uuid::new_v4())
        .with_metadata(metadata()
            .attr("can_remove_stubs_by_metadata-four", metadata()
                .attr("can_remove_stubs_by_metadata-five", "55555")
            )
            .list("can_remove_stubs_by_metadata-six", vec![1, 2, 3])
        )).unwrap();
    let stub2 = wire_mock.stub_for(get("/without-metadata")).unwrap();

    let json_path = "$..can_remove_stubs_by_metadata-four.can_remove_stubs_by_metadata-five";
    wire_mock.remove_stubs_by_metadata(matching_json_path(json_path)).unwrap();

    assert_eq!(wire_mock.remove_stub_mapping(stub1.id()).unwrap(), false);
    assert_eq!(wire_mock.remove_stub_mapping(stub2.id()).unwrap(), true);
}


fn create_wire_mock() -> WireMock {
    WireMockBuilder::new()
        .port(8181)
        .build()
}

fn print_json_value<T: serde::Serialize + ?Sized>(value: &T) {
    let json_string = serde_json::to_string_pretty(value).unwrap();
    println!("{}", json_string);
}
