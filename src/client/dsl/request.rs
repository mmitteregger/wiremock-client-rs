use crate::matching::{UrlPattern, RequestPatternBuilder};
use crate::http::RequestMethod;

pub fn url_equal_to<S>(url: S) -> UrlPattern
    where S: Into<String>
{
    UrlPattern::Url(url.into())
}

pub fn url_matching<S>(url_regex: S) -> UrlPattern
    where S: Into<String>
{
    UrlPattern::UrlPattern(url_regex.into())
}

pub fn url_path_equal_to<S>(url_path: S) -> UrlPattern
    where S: Into<String>
{
    UrlPattern::UrlPath(url_path.into())
}

pub fn url_path_matching<S>(url_path_regex: S) -> UrlPattern
    where S: Into<String>
{
    UrlPattern::UrlPathPattern(url_path_regex.into())
}

pub fn any_url() -> UrlPattern {
    UrlPattern::any()
}

pub fn get_requested_for<P>(url_pattern: P) -> RequestPatternBuilder
    where P: Into<UrlPattern>,
{
    RequestPatternBuilder::new(RequestMethod::GET, url_pattern.into())
}

pub fn post_requested_for<P>(url_pattern: P) -> RequestPatternBuilder
    where P: Into<UrlPattern>,
{
    RequestPatternBuilder::new(RequestMethod::POST, url_pattern.into())
}

pub fn put_requested_for<P>(url_pattern: P) -> RequestPatternBuilder
    where P: Into<UrlPattern>,
{
    RequestPatternBuilder::new(RequestMethod::PUT, url_pattern.into())
}

pub fn delete_requested_for<P>(url_pattern: P) -> RequestPatternBuilder
    where P: Into<UrlPattern>,
{
    RequestPatternBuilder::new(RequestMethod::DELETE, url_pattern.into())
}

pub fn patch_requested_for<P>(url_pattern: P) -> RequestPatternBuilder
    where P: Into<UrlPattern>,
{
    RequestPatternBuilder::new(RequestMethod::PATCH, url_pattern.into())
}

pub fn head_requested_for<P>(url_pattern: P) -> RequestPatternBuilder
    where P: Into<UrlPattern>,
{
    RequestPatternBuilder::new(RequestMethod::HEAD, url_pattern.into())
}

pub fn options_requested_for<P>(url_pattern: P) -> RequestPatternBuilder
    where P: Into<UrlPattern>,
{
    RequestPatternBuilder::new(RequestMethod::OPTIONS, url_pattern.into())
}

pub fn trace_requested_for<P>(url_pattern: P) -> RequestPatternBuilder
    where P: Into<UrlPattern>,
{
    RequestPatternBuilder::new(RequestMethod::TRACE, url_pattern.into())
}

pub fn any_requested_for<P>(url_pattern: P) -> RequestPatternBuilder
    where P: Into<UrlPattern>,
{
    RequestPatternBuilder::new(RequestMethod::ANY, url_pattern.into())
}
