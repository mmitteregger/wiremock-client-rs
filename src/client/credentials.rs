use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicCredentials {
    pub(crate) username: String,
    pub(crate) password: String,
}
