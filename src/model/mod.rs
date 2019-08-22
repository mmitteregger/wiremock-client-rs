use std::fmt::Debug;
use serde::{Serialize, Deserialize};

pub use pagination::{Meta, PageParams, PaginatedResult};
pub use get_global_settings_result::GetGlobalSettingsResult;
pub use list_stub_mappings_result::ListStubMappingsResult;
pub use single_served_stub_result::SingleServedStubResult;
pub use single_stub_mapping_result::SingleStubMappingResult;
pub use get_serve_events_result::GetServeEventsResult;

mod pagination;
mod get_global_settings_result;
mod list_stub_mappings_result;
mod single_served_stub_result;
mod single_stub_mapping_result;
mod get_serve_events_result;

pub trait SingleItemResult<T>: Debug + Serialize + Deserialize<'static> + Into<T> {
    fn item(&self) -> &T;
    fn item_mut(&mut self) -> &mut T;
}
