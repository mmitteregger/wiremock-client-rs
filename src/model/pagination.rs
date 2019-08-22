use std::fmt::Debug;
use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct PageParams {
    limit: u16,
    offset: u16,
}

pub trait PaginatedResult<T>: Debug + Serialize + Deserialize<'static> {
    fn selection(&self) -> &[T];
    fn meta(&self) -> &Meta;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    total: u16,
}

impl Meta {
    pub fn total(&self) -> u16 {
        self.total
    }
}
