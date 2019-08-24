use std::error::Error;

use wiremock_client::*;
use wiremock_client::stubbing::Scenario;

// Examples from: http://wiremock.org/docs/stateful-behaviour/
pub fn main() -> Result<(), Box<dyn Error>> {
    let wire_mock = WireMockBuilder::new()
        .port(8181) // If running on another port - the default is: 8080
        .build();

    // Scenarios
    wire_mock.stub_for(get(url_equal_to("/todo/items")).in_scenario("To do list")
        .when_scenario_state_is(Scenario::STARTED)
        .will_return(a_response()
            .with_body("<items>".to_string() +
                "   <item>Buy milk</item>" +
                "</items>")))?;

    wire_mock.stub_for(post(url_equal_to("/todo/items")).in_scenario("To do list")
        .when_scenario_state_is(Scenario::STARTED)
        .with_request_body(containing("Cancel newspaper subscription"))
        .will_return(a_response().with_status(201))
        .will_set_state_to("Cancel newspaper item added"))?;

    wire_mock.stub_for(get(url_equal_to("/todo/items")).in_scenario("To do list")
        .when_scenario_state_is("Cancel newspaper item added")
        .will_return(a_response()
            .with_body("<items>".to_string() +
                "   <item>Buy milk</item>" +
                "   <item>Cancel newspaper subscription</item>" +
                "</items>")))?;

    // Getting scenario state
    let _all_scenarios: Vec<Scenario> = wire_mock.get_scenarios()?;

    // Resetting scenarios
    wire_mock.reset_scenarios()?;

    // Reset mappings in case the integration tests are run afterwards
    wire_mock.reset_to_default_mappings()?;

    Ok(())
}
