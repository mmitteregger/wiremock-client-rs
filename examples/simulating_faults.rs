use std::error::Error;

use wiremock_client::*;
use wiremock_client::global::GlobalSettingsBuilder;
use wiremock_client::http::{DelayDistribution, Fault};

// Examples from: http://wiremock.org/docs/simulating-faults/
pub fn main() -> Result<(), Box<dyn Error>> {
    let wire_mock = WireMockBuilder::new()
        .port(8181) // If running on another port - the default is: 8080
        .build();

    // Per-stub fixed delays
    wire_mock.stub_for(get(url_equal_to("/delayed")).will_return(
        a_response()
            .with_status(200)
            .with_fixed_delay(2_000)))?;
    // Global fixed stub delays
    wire_mock.set_global_fixed_delay(500)?;

    // Per-stub random delays
    wire_mock.stub_for(get(url_equal_to("/random/delayed")).will_return(
        a_response()
            .with_status(200)
            .with_log_normal_random_delay(90.0, 0.1)))?;
    // Global random stub delays
    wire_mock.set_global_random_delay(DelayDistribution::Uniform { lower: 50, upper: 60 })?;

    // Available distributions
    // Lognormal delay
    let _ = DelayDistribution::LogNormal { median: 0.0, sigma: 0.0 };
    // Uniform delay
    let _ = DelayDistribution::Uniform { lower: 15, upper: 25 };

    // Chunked Dribble Delay
    wire_mock.stub_for(get("/chunked/delayed").will_return(
        a_response()
            .with_status(200)
            .with_body("Hello world!")
            .with_chunked_dribble_delay(5, 1_000)))?;

    // Bad responses
    wire_mock.stub_for(get(url_equal_to("/fault"))
        .will_return(a_response().with_fault(Fault::MALFORMED_RESPONSE_CHUNK)))?;

    // Reset mappings and reset global settings in case the integration tests are run afterwards
    wire_mock.reset_to_default_mappings()?;
    wire_mock.update_global_settings(&GlobalSettingsBuilder::new()
        .fixed_delay(None)
        .delay_distribution(None)
        .build())?;

    Ok(())
}
