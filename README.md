wiremock-client
===

[![Linux build status](https://travis-ci.org/mmitteregger/wiremock-client-rs.svg?branch=master)](https://travis-ci.org/mmitteregger/wiremock-client-rs)

A WireMock client for Rust that makes use the REST API of a running WireMock server
for stubbing and verifying.

The API is kept similar to the Java version, so that a code transformation is as painless as possible.

### Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
wiremock-client = { git = "https://github.com/mmitteregger/wiremock-client-rs" }
```

### Example

This crate is only useful in conjunction with a [running WireMock server](http://wiremock.org/docs/running-standalone/).\
If you have not already started WireMock as a standalone server, please do so before executing any example.

```rust
use std::error::Error;
use wiremock_client::*;

pub fn main() -> Result<(), Box<dyn Error>> {
    let wire_mock = WireMockBuilder::new()
        .port(8181) // If running on another port - the default is: 8080
        .build();

    wire_mock.stub_for(any(url_path_equal_to("/everything"))
        .with_header("Accept", containing("xml"))
        .with_cookie("session", matching(".*12345.*"))
        .with_query_param("search_term", equal_to("WireMock"))
        .with_basic_auth("jeff@example.com", "jeffteenjefftyjeff")
        .with_request_body(equal_to_xml("<search-results />"))
        .with_request_body(matching_xpath("//search-results"))
        .will_return(a_response()))?;

    Ok(())
}
```

More examples can be found in the examples directory.\
To run them, execute:
```ignore
cargo run --example <NAME>
```
The examples and integration tests (all tests in the tests directory) require a running WireMock server.
If nothing is changed, the server is expected to run on port 8181. 
Nothing else should be configured. 

An example setup can be found in the `before_script` section in `.travis.yml` 
which also installs and starts a WireMock instance.

### Documentation

The documentation for this crate is severely lacking.\
You are expected to read the [official WireMock documentation](http://wiremock.org/docs/).

A translation to this Rust code should be straightforward.\
If you are having trouble doing so please let me know.

### License

wiremock-client is licensed under either of the following, at your option:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
