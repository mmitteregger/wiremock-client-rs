use std::error::Error;

use wiremock_client::*;

// Examples from: http://wiremock.org/docs/request-matching/
pub fn main() -> Result<(), Box<dyn Error>> {
    let wire_mock = WireMockBuilder::new()
        .port(8181) // If running on another port - the default is: 8080
        .build();

    // Request Matching
    // Custom matching logic (http://wiremock.org/docs/extending-wiremock/#custom-request-matchers)
    // and multiparts are not yet implemented. Please open an issue if you require those.
    wire_mock.stub_for(any(url_path_equal_to("/everything"))
        .with_header("Accept", containing("xml"))
        .with_cookie("session", matching(".*12345.*"))
        .with_query_param("search_term", equal_to("WireMock"))
        .with_basic_auth("jeff@example.com", "jeffteenjefftyjeff")
        .with_request_body(equal_to_xml("<search-results />"))
        .with_request_body(matching_xpath("//search-results"))
        .will_return(a_response()))?;

    // URL matching
    // Equality matching on path and query
    wire_mock.stub_for(any(url_equal_to("/your/url?and=query")))?;
    // Regex matching on path and query
    wire_mock.stub_for(any(url_matching("/your/([a-z]*)\\?and=query")))?;
    // Equality matching on the path only
    wire_mock.stub_for(any(url_path_equal_to("/your/url")))?;
    // Regex matching on the path only
    wire_mock.stub_for(any(url_path_matching("/your/([a-z]*)")))?;

    // Matching other attributes
    // Equality
    wire_mock.stub_for(any(any_url())
        .with_header("Content-Type", equal_to("application/json")))?;
    // Case-insensitive equality
    wire_mock.stub_for(any(any_url())
        .with_header("Content-Type", equal_to_ignore_case("application/json")))?;
    // Binary Equality
    wire_mock.stub_for(any(any_url())
        .with_request_body(binary_equal_to(vec![1, 2, 3])))?;
    // Substring (contains)
    wire_mock.stub_for(any(any_url())
        .with_cookie("my_profile", containing("johnsmith@example.com")))?;
    // Regular expression
    wire_mock.stub_for(any(any_url())
        .with_query_param("search_term", matching("^(.*)wiremock([A-Za-z]+)$")))?;
    // Negative regular expression match
    wire_mock.stub_for(any(any_url())
        .with_query_param("search_term", not_matching("^(.*)wiremock([A-Za-z]+)$")))?;
    // JSON equality
    wire_mock.stub_for(any(any_url())
        .with_request_body(equal_to_json(r##""{ "total_results": 4 }""##)))?;
    // By default different array orderings and additional object attributes will trigger a non-match.
    // However, both of these conditions can be disabled individually
    wire_mock.stub_for(any(any_url())
        .with_request_body(equal_to_json(r##""{ "total_results": 4 }""##)
            .with_ignore_array_order(true)
            .with_ignore_extra_elements(true)))?;
    // JSON Path
    wire_mock.stub_for(any(any_url())
        .with_request_body(matching_json_path("$.name")))?;
    // for more JSON Path examples visit: http://wiremock.org/docs/request-matching/#json-path
    // XML equality
    wire_mock.stub_for(any(any_url())
        .with_request_body(equal_to_xml("<thing>Hello</thing>")))?;
    // Use of placeholders
    wire_mock.stub_for(any(any_url())
        .with_request_body(equal_to_xml("<message><id>${xmlunit.ignore}</id><content>Hello</content></message>")
            .with_enable_placeholders(true)
            .with_placeholder_delimiter_regexes(r##"\[\["##, r"]]")))?;
    // XPath
    wire_mock.stub_for(any(any_url())
        .with_request_body(matching_xpath("/todo-list[count(todo-item) = 3]")))?;
    // XPath namespaces
    wire_mock.stub_for(any(any_url())
        .with_request_body(matching_xpath("/stuff:outer/more:inner[.=111]")
            .with_xpath_namespace("stuff", "http://stuff.example.com")
            .with_xpath_namespace("more", "http://more.example.com")))?;

    // Absence
    wire_mock.stub_for(any(any_url())
        .with_cookie("session", absent())
        .with_query_param("search_term", absent())
        .with_header("X-Absent", absent()))?;

    // Multipart/form-data
    // .. is not yet implemented. Please open an issue if you require those.

    // Basic Authentication
    wire_mock.stub_for(get(url_equal_to("/basic-auth")).with_basic_auth("user", "pass"))?;

    // Reset in case the integration tests are run afterwards
    wire_mock.reset_to_default_mappings()?;

    Ok(())
}
