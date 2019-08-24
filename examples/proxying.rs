use std::error::Error;

use wiremock_client::*;

// Examples from: http://wiremock.org/docs/proxying/
pub fn main() -> Result<(), Box<dyn Error>> {
    let wire_mock = WireMockBuilder::new()
        .port(8181) // If running on another port - the default is: 8080
        .build();

    // Proxy stub mappings
    wire_mock.stub_for(get(url_matching("/other/service/.*"))
        .will_return(a_response().proxied_from("http://otherhost.com/approot")))?;

    // Proxy/intercept
    // Low priority catch-all proxies to otherhost.com by default
    wire_mock.stub_for(get(url_matching(".*")).at_priority(10)
        .will_return(a_response().proxied_from("http://otherhost.com")))?;
    // High priority stub will send a Service Unavailable response
    // if the specified URL is requested
    wire_mock.stub_for(get(url_equal_to("/api/override/123")).at_priority(1)
        .will_return(a_response().with_status(503)))?;

    // Additional headers
    // Inject user agent to trigger rendering of mobile version of website
    wire_mock.stub_for(get(url_matching(".*"))
        .will_return(a_response()
            .proxied_from("http://otherhost.com")
            .with_additional_request_header("User-Agent", "Mozilla/5.0 (iPhone; U; CPU iPhone)")))?;

    // Reset mappings and reset global settings in case the integration tests are run afterwards
    wire_mock.reset_to_default_mappings()?;

    Ok(())
}
