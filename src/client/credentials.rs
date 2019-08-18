use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicCredentials {
    pub username: String,
    pub password: String,
}
