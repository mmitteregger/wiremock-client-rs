use serde::{Deserialize, Serialize};

use crate::http::multi_value::MultiValue;

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Cookie {
    cookies: MultiValue,
}
