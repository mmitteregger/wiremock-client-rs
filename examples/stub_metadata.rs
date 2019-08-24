use std::error::Error;

use wiremock_client::*;
use wiremock_client::stubbing::StubMapping;

// Examples from: http://wiremock.org/docs/stub-metadata/
pub fn main() -> Result<(), Box<dyn Error>> {
    let wire_mock = WireMockBuilder::new()
        .port(8181) // If running on another port - the default is: 8080
        .build();

    // Adding metadata to stubs
    wire_mock.stub_for(get("/with-metadata")
        .with_metadata(metadata()
            .attr("singleItem", 1234)
            .list("listItem", vec![1, 2, 3, 4])
            .attr("nestedObject", metadata()
                .attr("innerItem", "Hello"),
            )
        ))?;

    // Search for stubs by metadata
    let _stubs: Vec<StubMapping> =
        wire_mock.find_stubs_by_metadata(matching_json_path("$.singleItem"))?;

    // Delete stubs by metadata
    wire_mock.remove_stubs_by_metadata(matching_json_path("$.singleItem"))?;

    Ok(())
}
