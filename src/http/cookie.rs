use serde::{Deserialize, Serialize};

use crate::http::multi_value::MultiValue;

#[derive(Debug, Serialize, Deserialize)]
pub struct Cookie {
    key: String,
    values: Vec<String>,
}

impl Cookie {
    pub fn new(key: String, values: Vec<String>) -> Cookie {
        Cookie {
            key,
            values,
        }
    }

    fn check_present(&self) {
        if !self.is_present() {
            panic!("no value for {}", self.key);
        }
    }
}

impl MultiValue for Cookie {
    fn is_present(&self) -> bool {
        !self.values.is_empty()
    }

    fn key(&self) -> &str {
        &self.key
    }

    fn first_value(&self) -> &str {
        self.check_present();
        &self.values[0]
    }

    fn values(&self) -> &[String] {
        self.check_present();
        &self.values
    }

    fn is_single_valued(&self) -> bool {
        self.values.len() == 1
    }

    fn contains_value(&self, expected_value: &str) -> bool {
        self.values.iter().any(|value| value == expected_value)
    }
}
