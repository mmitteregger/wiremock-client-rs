use uuid::Uuid;

use wiremock_client::{a_response, containing, equal_to, get, get_requested_for, no_content, ok, url_equal_to, url_path_equal_to, WireMock, WireMockBuilder, any_url, any, post_requested_for};
use wiremock_client::global::GlobalSettingsBuilder;
use wiremock_client::http::DelayDistribution;
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
fn list_all_stub_mappings() {
    let wire_mock = create_wire_mock();
    let stub_mappings_result = wire_mock.list_all_stub_mappings().unwrap();
    print_json_value(&stub_mappings_result);
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

    reqwest::Client::new()
        .post(&format!("http://localhost:8181{}", &url))
        .send()
        .unwrap();

    let near_misses = wire_mock.find_near_misses_for_unmatched_requests().unwrap();
    print_json_value(&near_misses);

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
#[ignore = "updating the global settings my interere with other tests"]
fn update_and_reset_global_settings() {
    let wire_mock = create_wire_mock();

    let default_global_settings = wire_mock.get_global_settings()
        .unwrap()
        .into_settings();

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
    let get_global_settings_result = wire_mock.get_global_settings().unwrap();
    print_json_value(&get_global_settings_result);
}

#[test]
#[ignore = "shutting down the server lets all subsequent tests fail"]
fn shutdown_server() {
    let wire_mock = create_wire_mock();
    wire_mock.shutdown_server().unwrap();
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
