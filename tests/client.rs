use uuid::Uuid;

use wiremock_client::{WireMock, WireMockBuilder, get, url_equal_to, ok, containing};
use wiremock_client::global::GlobalSettingsBuilder;
use wiremock_client::http::DelayDistribution;
use wiremock_client::matching::{ContentPattern, UrlPattern, EqualToPattern};

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

    let stub_mapping_removed = wire_mock.remove_stub_mapping(&stub_mapping.id).unwrap();
    assert_eq!(stub_mapping_removed, true);
}

#[test]
fn stub_for_get_str() {
    let wire_mock = create_wire_mock();

    let stub_mapping = wire_mock.stub_for(get("/some/thing")).unwrap();

    let stub_mapping_removed = wire_mock.remove_stub_mapping(&stub_mapping.id).unwrap();
    assert_eq!(stub_mapping_removed, true);
}

#[test]
fn stub_for_get_string() {
    let wire_mock = create_wire_mock();

    let stub_mapping = wire_mock.stub_for(get("/some/thing".to_string())).unwrap();

    let stub_mapping_removed = wire_mock.remove_stub_mapping(&stub_mapping.id).unwrap();
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

    let stub_mapping_removed = wire_mock.remove_stub_mapping(&stub_mapping.id).unwrap();
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
    edited_stub_mapping.name = Some("Test: edit_stub_mapping (edited)".to_string());
    edited_stub_mapping.request.url = UrlPattern::UrlPath("test".to_string());
    edited_stub_mapping.request.body_patterns = vec![
        ContentPattern::EqualTo(EqualToPattern::new("edit_stub_mapping".to_string(), Some(false))),
    ];
    edited_stub_mapping.response.status = 204;
    edited_stub_mapping.response.status_message = Some("No content".to_string());
    edited_stub_mapping.response.body = None;
    edited_stub_mapping.priority = Some(4);

    wire_mock.edit_stub_mapping(&edited_stub_mapping).unwrap();

    let stub_mapping_removed = wire_mock.remove_stub_mapping(&edited_stub_mapping.id).unwrap();
    assert_eq!(stub_mapping_removed, true);
}

#[test]
fn remove_non_existent_stub_mapping() {
    let wire_mock = create_wire_mock();

    let stub_mapping_removed = wire_mock.remove_stub_mapping(&Uuid::new_v4()).unwrap();
    assert_eq!(stub_mapping_removed, false);
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
