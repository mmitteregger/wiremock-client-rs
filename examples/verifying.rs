use std::error::Error;

use wiremock_client::*;
use wiremock_client::stubbing::ServeEvent;
use wiremock_client::verification::{LoggedRequest, NearMiss};

// Examples from: http://wiremock.org/docs/verifying/
pub fn main() -> Result<(), Box<dyn Error>> {
    let wire_mock = WireMockBuilder::new()
        .port(8181) // If running on another port - the default is: 8080
        .build();

    // ignore_panic below because those verifications will likely fail (panic) without any requests!

    // Verifying
    ignore_panic(|| {
        wire_mock.verify(post_requested_for(url_equal_to("/verify/this"))
            .with_header("Content-Type", equal_to("text/xml")));
    });
    // To check for a precise number of requests matching the criteria
    ignore_panic(|| {
        wire_mock.verify_count(3, post_requested_for(url_equal_to("/three/times")));
    });
    // Or you can use some more advanced comparison operators
    ignore_panic(|| {
        wire_mock.verify_count(less_than(5), post_requested_for(url_equal_to("/many")));
    });
    ignore_panic(|| {
        wire_mock.verify_count(less_than_or_exactly(5), post_requested_for(url_equal_to("/many")));
    });
    ignore_panic(|| {
        wire_mock.verify_count(exactly(5), post_requested_for(url_equal_to("/many")));
    });
    ignore_panic(|| {
        wire_mock.verify_count(more_than_or_exactly(5), post_requested_for(url_equal_to("/many")));
    });
    ignore_panic(|| {
        wire_mock.verify_count(more_than(5), post_requested_for(url_equal_to("/many")));
    });

    // Querying the request journal
    // Getting all requests
    let _all_serve_events: Vec<ServeEvent> = wire_mock.get_serve_events()?;
    // Criteria queries
    let _requests: Vec<LoggedRequest> = wire_mock.find(put_requested_for(url_matching("/api/.*")))?;

    // Resetting the request journal
    wire_mock.reset_requests()?;

    // Finding unmatched requests
    let unmatched: Vec<LoggedRequest> = wire_mock.find_unmatched()?;

    // Near misses
    // .. representing stub mappings closest to the specified logged request
    if let Some(logged_request) = unmatched.get(0) {
        let _near_misses: Vec<NearMiss> = wire_mock.find_near_misses_for_request(logged_request)?;
    }
    // .. representing stub mappings closest to the specified request
    let _near_misses: Vec<NearMiss> = wire_mock.find_near_misses_for(
        get_requested_for(url_equal_to("/thing-url"))
            .with_request_body(containing("thing"))
    )?;
    // top near misses for every unmatched request
    let _near_misses: Vec<NearMiss> = wire_mock.find_near_misses_for_unmatched_requests()?;

    Ok(())
}

pub fn ignore_panic<F: FnOnce()>(f: F) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f()));
}
