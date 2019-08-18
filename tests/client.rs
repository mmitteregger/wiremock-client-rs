use uuid::Uuid;

use wiremock_client::client::WireMockClient;
use wiremock_client::global::GlobalSettingsBuilder;
use wiremock_client::http::{DelayDistribution, RequestMethod, ResponseDefinition, Body};
use wiremock_client::matching::{ContentPattern, RequestPattern, Url};
use wiremock_client::stubbing::StubMapping;

macro_rules! string_map {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(string_map!(@single $rest)),*]));

    ($($key:expr => $value:expr,)+) => { string_map!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            let _cap = string_map!(@count $($key),*);
            let mut _map = ::std::collections::HashMap::with_capacity(_cap);
            $(
                let _ = _map.insert(String::from($key), String::from($value));
            )*
            _map
        }
    };
}

#[test]
fn add_stub_mapping() {
    let wire_mock_client = WireMockClient::default();

    let stub_mapping = StubMapping {
        id: Uuid::new_v4(),
        name: "Test: add_stub_mapping".to_string(),
        request: RequestPattern {
            url: Url::Url("test".to_string()),
            method: RequestMethod::GET,
            query_params: Default::default(),
            headers: Default::default(),
            cookies: Default::default(),
            basic_auth_credentials: None,
            body_patterns: vec![
                ContentPattern::Contains {
                    contains: "add_stub_mapping".to_string()
                }
            ],
        },
        response: ResponseDefinition {
            status: 200,
            status_message: "OK".to_string(),
            body: Some(Body::String("Hello world!".to_string())),
            headers: Default::default(),
            fixed_delay_milliseconds: None,
            proxy_base_url: None,
            fault: None,
            transformers: vec![],
            transformer_parameters: Default::default(),
            from_configured_stub: false,
        },
        persistent: false,
        priority: 0,
        scenario_name: None,
        required_scenario_state: None,
        new_scenario_state: None,
        post_serve_actions: Default::default(),
        metadata: Default::default(),
    };

    wire_mock_client.add_stub_mapping(&stub_mapping).unwrap();

    wire_mock_client.reset_mappings().unwrap();
}

#[test]
fn list_all_stub_mappings() {
    let wire_mock_client = WireMockClient::default();
    let stub_mappings_result = wire_mock_client.list_all_stub_mappings().unwrap();
    dbg!(stub_mappings_result);
}

#[test]
fn reset_all() {
    let wire_mock_client = WireMockClient::default();
    wire_mock_client.reset_all().unwrap();
}

#[test]
fn reset_requests() {
    let wire_mock_client = WireMockClient::default();
    wire_mock_client.reset_requests().unwrap();
}

#[test]
fn reset_scenarios() {
    let wire_mock_client = WireMockClient::default();
    wire_mock_client.reset_scenarios().unwrap();
}

#[test]
fn reset_mappings() {
    let wire_mock_client = WireMockClient::default();
    wire_mock_client.reset_mappings().unwrap();
}

#[test]
fn reset_to_default_mappings() {
    let wire_mock_client = WireMockClient::default();
    wire_mock_client.reset_to_default_mappings().unwrap();
}

#[test]
fn update_global_settings() {
    let wire_mock_client = WireMockClient::default();

    let default_global_settings = wire_mock_client.get_global_settings()
        .unwrap()
        .into_settings();

    let global_settings = GlobalSettingsBuilder::new()
        .fixed_delay(Some(50))
        .delay_distribution(Some(DelayDistribution::Uniform { lower: 50, upper: 60 }))
        .extended(string_map! { "e1" => "v1", "e2" => "v2" })
        .build();

    wire_mock_client.update_global_settings(&global_settings).unwrap();

    wire_mock_client.update_global_settings(&default_global_settings).unwrap();
}

#[test]
fn get_global_settings() {
    let wire_mock_client = WireMockClient::default();
    let get_global_settings_result = wire_mock_client.get_global_settings().unwrap();
    dbg!(get_global_settings_result);
}

#[test]
#[ignore = "shutting down the server lets all subsequent tests fail"]
fn shutdown_server() {
    let wire_mock_client = WireMockClient::default();
    wire_mock_client.shutdown_server().unwrap();
}
