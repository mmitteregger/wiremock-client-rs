/*!
A WireMock client for Rust that makes use the REST API of a running WireMock server
for stubbing and verifying.

The API is kept similar to the Java version, so that a code transformation is as painless as possible.

# Setup
Add this to your `Cargo.toml`:

```toml
[dependencies]
wiremock-client = { git = "https://github.com/mmitteregger/wiremock-client-rs" }
```

# Example

This crate is only useful in conjunction with a [running WireMock server](http://wiremock.org/docs/running-standalone/).\
If you have not already started WireMock as a standalone server, please do so before executing any example.

```rust,no_run
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

# Documentation

The documentation for this crate is severely lacking.\
You are expected to read the [official WireMock documentation](http://wiremock.org/docs/).

A translation to this Rust code should be straightforward.\
If you are having trouble doing so please let me know.

# Code structure

The modules structure mostly matches the package structure of the Java code.
Some notable differences are:
* The client module is re-exported in the root, because this crate already **is a client only**.
* Static DSL functions in WireMock are free standing functions that are accessable from the root path.
* Some additional DSL functions are added to the root path:
    * `stub_import`
    * `metadata`
* `HttpHeaders` and `HttpHeader` are replaced with `HeaderMap` and `HeaderValue` of the `http` crate.
* Some Java interfaces have been replaced with nonexhaustive enums instead of traits.
  This is hopefully is not a problem, because I expect those types only to be used via the provided DSL functions.
  Let me know if this assumption is wrong.
* Overloaded methods have been replaced with generics were possible.
*/

pub use client::*;

pub mod global;
pub mod security;
pub mod matching;
pub mod model;
pub mod stubbing;
pub mod http;
mod client;
pub mod common;
pub mod extension;
pub mod verification;
mod serde;
