use http::StatusCode;

use crate::client::ResponseDefinitionBuilder;
use crate::http::Body;

pub fn a_response() -> ResponseDefinitionBuilder {
    ResponseDefinitionBuilder::new()
}

pub fn ok() -> ResponseDefinitionBuilder {
    status(StatusCode::OK.as_u16())
}

pub fn ok_with_body<B>(body: B) -> ResponseDefinitionBuilder
    where B: Into<Body>,
{
    ok().with_body(body)
}

pub fn ok_for_content_type<S, B>(content_type: S, body: B) -> ResponseDefinitionBuilder
    where S: Into<String>,
          B: Into<Body>,
{
    ok()
        .with_header(reqwest::header::CONTENT_TYPE.to_string(), content_type.into())
        .with_body(body)
}

pub fn ok_json<B>(body: B) -> ResponseDefinitionBuilder
    where B: Into<Body>,
{
    ok_for_content_type("application/json", body)
}

pub fn ok_xml<B>(body: B) -> ResponseDefinitionBuilder
    where B: Into<Body>,
{
    ok_for_content_type("application/xml", body)
}

pub fn ok_text_xml<B>(body: B) -> ResponseDefinitionBuilder
    where B: Into<Body>,
{
    ok_for_content_type("text/xml", body)
}

pub fn created() -> ResponseDefinitionBuilder {
    status(StatusCode::CREATED.as_u16())
}

pub fn no_content() -> ResponseDefinitionBuilder {
    status(StatusCode::NO_CONTENT.as_u16())
}

pub fn permanent_redirect<S>(location: S) -> ResponseDefinitionBuilder
    where S: AsRef<str>,
{
    status(StatusCode::PERMANENT_REDIRECT.as_u16())
        .with_header(reqwest::header::LOCATION.to_string(), location)
}

pub fn temporary_redirect<S>(location: S) -> ResponseDefinitionBuilder
    where S: AsRef<str>,
{
    status(StatusCode::TEMPORARY_REDIRECT.as_u16())
        .with_header(reqwest::header::LOCATION.to_string(), location)
}

pub fn see_other<S>(location: S) -> ResponseDefinitionBuilder
    where S: AsRef<str>,
{
    status(StatusCode::SEE_OTHER.as_u16())
        .with_header(reqwest::header::LOCATION.to_string(), location)
}

pub fn bad_request() -> ResponseDefinitionBuilder {
    status(StatusCode::BAD_REQUEST.as_u16())
}

pub fn bad_request_entity() -> ResponseDefinitionBuilder {
    unprocessable_entity()
}

pub fn unprocessable_entity() -> ResponseDefinitionBuilder {
    status(StatusCode::UNPROCESSABLE_ENTITY.as_u16())
}

pub fn unauthorized() -> ResponseDefinitionBuilder {
    status(StatusCode::UNAUTHORIZED.as_u16())
}

pub fn forbidden() -> ResponseDefinitionBuilder {
    status(StatusCode::FORBIDDEN.as_u16())
}

pub fn not_found() -> ResponseDefinitionBuilder {
    status(StatusCode::NOT_FOUND.as_u16())
}

pub fn server_error() -> ResponseDefinitionBuilder {
    status(StatusCode::INTERNAL_SERVER_ERROR.as_u16())
}

pub fn service_unavailable() -> ResponseDefinitionBuilder {
    status(StatusCode::SERVICE_UNAVAILABLE.as_u16())
}

pub fn status(status: u16) -> ResponseDefinitionBuilder {
    ResponseDefinitionBuilder::new().with_status(status)
}
