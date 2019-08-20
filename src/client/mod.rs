use reqwest::{Method, RequestBuilder, Response, StatusCode};
use reqwest::header::HeaderValue;
use serde::Serialize;

pub use builder::WireMockBuilder;
pub use credentials::BasicCredentials;

use crate::client::builder::MappingBuilder;
use crate::global::GlobalSettings;
use crate::http::Result;
use crate::matching::UrlPattern;
use crate::model::{GetGlobalSettingsResult, ListStubMappingsResult};
use crate::security::ClientAuthenticator;
use crate::stubbing::StubMapping;
use uuid::Uuid;

pub(crate) mod builder;
mod credentials;

pub struct WireMock {
    client: reqwest::Client,
    scheme: String,
    host: String,
    port: u16,
    url_path_prefix: String,
    host_header: Option<HeaderValue>,
    authenticator: Box<dyn ClientAuthenticator>,
}

impl Default for WireMock {
    fn default() -> WireMock {
        WireMockBuilder::new().build()
    }
}

impl From<WireMockBuilder> for WireMock {
    fn from(builder: WireMockBuilder) -> WireMock {
        builder.build()
    }
}

impl WireMock {
    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

//    router.add(POST, "/mappings/save", SaveMappingsTask.class);
//    router.add(POST, "/mappings/reset", ResetToDefaultMappingsTask.class);
//    router.add(GET,  "/mappings/{id}", GetStubMappingTask.class);
//    router.add(PUT,  "/mappings/{id}", EditStubMappingTask.class);
//    router.add(DELETE, "/mappings/{id}", RemoveStubMappingTask.class);
//    router.add(POST, "/mappings/find-by-metadata", FindStubMappingsByMetadataTask.class);
//    router.add(POST, "/mappings/remove-by-metadata", RemoveStubMappingsByMetadataTask.class);
//    router.add(POST, "/mappings/import", ImportStubMappingsTask.class);
//
//    router.add(GET, "/files", GetAllStubFilesTask.class);
//    router.add(PUT, "/files/{filename}", EditStubFileTask.class);
//    router.add(DELETE, "/files/{filename}", DeleteStubFileTask.class);
//
//    router.add(GET, "/scenarios", GetAllScenariosTask.class);
//    router.add(POST, "/scenarios/reset", ResetScenariosTask.class);
//
//    router.add(GET,  "/requests", GetAllRequestsTask.class);
//    router.add(DELETE,  "/requests", ResetRequestsTask.class);
//    router.add(POST, "/requests/reset", OldResetRequestsTask.class);  // Deprecated
//    router.add(POST, "/requests/count", GetRequestCountTask.class);
//    router.add(POST, "/requests/find", FindRequestsTask.class);
//    router.add(GET,  "/requests/unmatched", FindUnmatchedRequestsTask.class);
//    router.add(GET,  "/requests/unmatched/near-misses", FindNearMissesForUnmatchedTask.class);
//    router.add(GET,  "/requests/{id}", GetServedStubTask.class);
//
//    router.add(POST, "/recordings/snapshot", SnapshotTask.class);
//    router.add(POST, "/recordings/start", StartRecordingTask.class);
//    router.add(POST, "/recordings/stop", StopRecordingTask.class);
//    router.add(GET,  "/recordings/status", GetRecordingStatusTask.class);
//    router.add(GET,  "/recorder", GetRecordingsIndexTask.class);
//
//    router.add(POST, "/near-misses/request", FindNearMissesForRequestTask.class);
//    router.add(POST, "/near-misses/request-pattern", FindNearMissesForRequestPatternTask.class);
//
//    router.add(GET, "/settings", GetGlobalSettingsTask.class);
//    router.add(PUT, "/settings", GlobalSettingsUpdateTask.class);
//    router.add(POST, "/settings", GlobalSettingsUpdateTask.class);
//    router.add(PATCH, "/settings/extended", PatchExtendedSettingsTask.class);
//
//    router.add(POST, "/shutdown", ShutdownServerTask.class);
//
//    router.add(GET, "/docs/swagger", GetSwaggerSpecTask.class);
//    router.add(GET, "/docs", GetDocIndexTask.class);

    pub fn given_that<S: Into<StubMapping>>(&self, stub_mapping: S) -> Result<StubMapping> {
        let stub_mapping = stub_mapping.into();
        self.add_stub_mapping(&stub_mapping)?;
        Ok(stub_mapping)
    }

    pub fn stub_for<S: Into<StubMapping>>(&self, stub_mapping: S) -> Result<StubMapping> {
        self.given_that(stub_mapping)
    }

    pub fn add_stub_mapping(&self, stub_mapping: &StubMapping) -> Result<()> {
        self.send_json_request(Method::POST, "/mappings", stub_mapping)
            .map(|_| ())
    }

    pub fn edit_stub_mapping(&self, stub_mapping: &StubMapping) -> Result<()> {
        self.send_json_request(Method::PUT, &format!("/mappings/{}", stub_mapping.id), stub_mapping)
            .map(|_| ())
    }

    pub fn remove_stub_mapping(&self, id: &Uuid) -> Result<bool> {
        self.send_empty_request(Method::DELETE, &format!("/mappings/{}", id))
            .map(|_| true)
            .or_else(|error| {
                if let Some(status_code) = error.status() {
                    if status_code == StatusCode::NOT_FOUND {
                        return Ok(false);
                    }
                }

                Err(error)
            })
    }

    pub fn list_all_stub_mappings(&self) -> Result<ListStubMappingsResult> {
        self.send_empty_request(Method::GET, "/")
            .and_then(|mut response| response.json::<ListStubMappingsResult>())
    }

    pub fn reset_all(&self) -> Result<()> {
        self.send_empty_request(Method::POST, "/reset")
            .map(|_| ())
    }

    pub fn reset_requests(&self) -> Result<()> {
        self.send_empty_request(Method::DELETE, "/requests")
            .map(|_| ())
    }

    pub fn reset_scenarios(&self) -> Result<()> {
        self.send_empty_request(Method::POST, "/scenarios/reset")
            .map(|_| ())
    }

    pub fn reset_mappings(&self) -> Result<()> {
        self.send_empty_request(Method::DELETE, "/mappings")
            .map(|_| ())
    }

    pub fn reset_to_default_mappings(&self) -> Result<()> {
        self.send_empty_request(Method::POST, "/mappings/reset")
            .map(|_| ())
    }

    pub fn shutdown_server(&self) -> Result<()> {
        self.send_empty_request(Method::POST, "/shutdown")
            .map(|_| ())
    }

    pub fn update_global_settings(&self, global_settings: &GlobalSettings) -> Result<()> {
        self.send_json_request(Method::POST, "/settings", global_settings)
            .map(|_| ())
    }

    pub fn get_global_settings(&self) -> Result<GetGlobalSettingsResult> {
        self.send_empty_request(Method::GET, "/settings")
            .and_then(|mut response| response.json::<GetGlobalSettingsResult>())
    }


    fn send_empty_request(&self, method: Method, path: &str) -> Result<Response> {
        let request = self.create_request(method, path);

        request.send()
            .and_then(|response| response.error_for_status())
    }

    fn send_json_request<T>(&self, method: Method, path: &str, json: &T) -> Result<Response>
        where T: Serialize + ?Sized
    {
        let request = self.create_request(method, path);

        request.json(json)
            .send()
            .and_then(|response| response.error_for_status())
    }

    fn create_request(&self, method: Method, path: &str) -> RequestBuilder {
        let url = format!("{}://{}:{}{}/__admin{}",
            self.scheme, self.host, self.port, self.url_path_prefix, path);

        let mut request = self.client.request(method, &url);

        if let Some(host_header) = self.host_header.as_ref() {
            request = request.header(reqwest::header::HOST, host_header);
        };

        for (header_name, header_value) in self.authenticator.generate_auth_headers().iter() {
            request = request.header(header_name, header_value);
        };

        request
    }
}

pub fn get<P>(url: P) -> MappingBuilder
    where P: Into<UrlPattern>
{
    MappingBuilder {}
}

pub fn url_equal_to<S>(url: S) -> UrlPattern
    where S: Into<String>
{
    UrlPattern::Url(url.into())
}
