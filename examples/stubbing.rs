use std::error::Error;

use uuid::Uuid;

use wiremock_client::*;
use wiremock_client::stubbing::StubMapping;
use wiremock_client::http::StatusCode;

// Examples from: http://wiremock.org/docs/stubbing/
pub fn main() -> Result<(), Box<dyn Error>> {
    let wire_mock = WireMockBuilder::new()
        .port(8181) // If running on another port - the default is: 8080
        .build();

    // Basic stubbing
    wire_mock.stub_for(get(url_equal_to("/some/thing"))
        .will_return(a_response()
            .with_header("Content-Type", "text/plain")
            .with_body("Hello world!")))?;

    // Shortcuts
    wire_mock.stub_for(get("/some/thing")
        .will_return(a_response().with_status(StatusCode::OK)))?;

    wire_mock.stub_for(delete("/fine")
        .will_return(ok()))?;

    wire_mock.stub_for(get("/fine-with-body")
        .will_return(ok_with_body("body content")))?;

    wire_mock.stub_for(get("/json")
        .will_return(ok_json(r##"{ "message": "Hello" }"##)))?;

    wire_mock.stub_for(post("/redirect")
        .will_return(temporary_redirect("/new/place")))?;

    wire_mock.stub_for(post("/sorry-no")
        .will_return(unauthorized()))?;

    wire_mock.stub_for(put("/status-only")
        .will_return(status(StatusCode::IM_A_TEAPOT)))?;

    // Setting the response status message
    wire_mock.stub_for(get(url_equal_to("/some/thing"))
        .will_return(a_response()
            .with_status(StatusCode::OK)
            .with_status_message("Everything was just fine!")
            .with_header("Content-Type", "text/plain")))?;

    // Stub priority
    // Catch-all case
    wire_mock.stub_for(get(url_matching("/api/.*")).at_priority(5)
        .will_return(a_response().with_status(StatusCode::UNAUTHORIZED)))?;
    // Specific case
    wire_mock.stub_for(get(url_equal_to("/api/specific-resource")).at_priority(1) // 1 is highest
        .will_return(a_response()
            .with_status(StatusCode::OK)
            .with_body("Resource state")))?;

    // Sending response headers
    wire_mock.stub_for(get(url_equal_to("/whatever"))
        .will_return(a_response()
            .with_status(StatusCode::OK)
            .with_header("Content-Type", "application/json")
            .with_header("Set-Cookie", "session_id=91837492837")
            .with_header("Set-Cookie", "split_test_group=B") // You can call withHeader more than once for the same header if multiple values are required
            .with_header("Cache-Control", "no-cache")))?;

    // Specifying the response body
    wire_mock.stub_for(get(url_equal_to("/body"))
        .will_return(a_response()
            .with_body("Literal text to put in the body")))?;
    // Response body from file under the __files directory of the WireMock server
    wire_mock.stub_for(get(url_equal_to("/body-file"))
        .will_return(a_response()
            .with_body_file("path/to/myfile.xml")))?;
    // Binary body
    wire_mock.stub_for(get(url_equal_to("/binary-body"))
        .will_return(a_response()
            .with_body(vec![1, 2, 3, 4])))?;

    // Default response for unmapped requests
    wire_mock.stub_for(any(any_url())
        .at_priority(10)
        .will_return(a_response()
            .with_status(StatusCode::NOT_FOUND)
            .with_body(r##"{"status":"Error","message":"Endpoint not found"}"##)))?;

    // Saving stubs
    // wire_mock.save_mappings()?;

    // Editing stubs
    let id = Uuid::new_v4();
    wire_mock.stub_for(get(url_equal_to("/edit-this"))
        .with_id(id)
        .will_return(a_response()
            .with_body("Original")))?;
    wire_mock.stub_for(get(url_equal_to("/edit-this"))
        .with_id(id)
        .will_return(a_response()
            .with_body("Modified")))?;

    // Removing stubs
    let stub_mapping = wire_mock.stub_for(get(url_equal_to("/delete-me"))
        .will_return(a_response().with_status(StatusCode::OK)))?;
    // Do things with the stub
    wire_mock.remove_stub(stub_mapping)?;

    // Getting all currently registered stub mappings
    let _stub_mappings: Vec<StubMapping> = wire_mock.list_stub_mappings()?;

    // Getting a single stub mapping by ID
    let id = Uuid::new_v4(); // or retrieved via stub_mapping.id()
    let _stub_mapping: Option<StubMapping> = wire_mock.get_stub_mapping(&id)?;

    // Bulk importing stubs
    wire_mock.import_stubs(stub_import()
        .stub(get("/one").will_return(ok()))
        .stub(post("/two").will_return(ok_with_body("Body content")))
        .stub(put("/three").will_return(ok()))
        .ignore_existing()
        .do_not_delete_existing_stubs())?;

    // Reset
    // wire_mock.reset_all()?;
    wire_mock.reset_to_default_mappings()?;

    Ok(())
}
