use std::fmt::Debug;

use serde::{Deserialize, Serialize};

pub trait MultiValue: Debug + Serialize + Deserialize<'static> {
    fn is_present(&self) -> bool;

    fn key(&self) -> &str;

    fn first_value(&self) -> &str;

    fn values(&self) -> &[String];

    fn is_single_valued(&self) -> bool;

    fn contains_value(&self, expected_value: &str) -> bool;
}
