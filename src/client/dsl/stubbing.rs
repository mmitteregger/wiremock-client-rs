use crate::client::MappingBuilder;
use crate::matching::UrlPattern;
use crate::http::RequestMethod;

pub fn get<P>(url_pattern: P) -> MappingBuilder
    where P: Into<UrlPattern>,
{
    MappingBuilder::new(RequestMethod::GET, url_pattern.into())
}

pub fn post<P>(url_pattern: P) -> MappingBuilder
    where P: Into<UrlPattern>,
{
    MappingBuilder::new(RequestMethod::POST, url_pattern.into())
}

pub fn put<P>(url_pattern: P) -> MappingBuilder
    where P: Into<UrlPattern>,
{
    MappingBuilder::new(RequestMethod::PUT, url_pattern.into())
}

pub fn delete<P>(url_pattern: P) -> MappingBuilder
    where P: Into<UrlPattern>,
{
    MappingBuilder::new(RequestMethod::DELETE, url_pattern.into())
}

pub fn patch<P>(url_pattern: P) -> MappingBuilder
    where P: Into<UrlPattern>,
{
    MappingBuilder::new(RequestMethod::PATCH, url_pattern.into())
}

pub fn head<P>(url_pattern: P) -> MappingBuilder
    where P: Into<UrlPattern>,
{
    MappingBuilder::new(RequestMethod::HEAD, url_pattern.into())
}

pub fn options<P>(url_pattern: P) -> MappingBuilder
    where P: Into<UrlPattern>,
{
    MappingBuilder::new(RequestMethod::OPTIONS, url_pattern.into())
}

pub fn trace<P>(url_pattern: P) -> MappingBuilder
    where P: Into<UrlPattern>,
{
    MappingBuilder::new(RequestMethod::TRACE, url_pattern.into())
}

pub fn any<P>(url_pattern: P) -> MappingBuilder
    where P: Into<UrlPattern>,
{
    MappingBuilder::new(RequestMethod::ANY, url_pattern.into())
}

pub fn request<M, P>(method: M, url_pattern: P) -> MappingBuilder
    where M: Into<RequestMethod>,
          P: Into<UrlPattern>,
{
    MappingBuilder::new(method.into(), url_pattern.into())
}

//public static MappingBuilder requestMatching(String customRequestMatcherName) {
//return new BasicMappingBuilder(customRequestMatcherName, Parameters.empty());
//}
//
//public static MappingBuilder requestMatching(String customRequestMatcherName, Parameters parameters) {
//return new BasicMappingBuilder(customRequestMatcherName, parameters);
//}
//
//public static MappingBuilder requestMatching(ValueMatcher<Request> requestMatcher) {
//return new BasicMappingBuilder(requestMatcher);
//}

pub fn proxy_all_to<S>(url: S) -> MappingBuilder
    where S: Into<String>,
{
    any(crate::any_url()).will_return(crate::a_response().proxied_from(url))
}
