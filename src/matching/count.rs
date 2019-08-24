use std::fmt;
use std::convert::TryInto;

pub trait CountMatchingMode {
    fn friendly_name(&self, ) -> &'static str;
    fn is_match(&self, actual: u32) -> bool;
}

#[derive(Debug, Copy, Clone)]
pub enum CountMatchingStrategy {
    LessThan(u32),
    LessThanOrEqual(u32),
    EqualTo(u32),
    GreaterThanOrEqual(u32),
    GreaterThan(u32),
}

impl CountMatchingStrategy {
    fn expected(&self) -> u32 {
        match *self {
            CountMatchingStrategy::LessThan(expected) => expected,
            CountMatchingStrategy::LessThanOrEqual(expected) => expected,
            CountMatchingStrategy::EqualTo(expected) => expected,
            CountMatchingStrategy::GreaterThanOrEqual(expected) => expected,
            CountMatchingStrategy::GreaterThan(expected) => expected,
        }
    }
}

impl CountMatchingMode for CountMatchingStrategy {
    fn friendly_name(&self) -> &'static str {
        match *self {
            CountMatchingStrategy::LessThan(_) => "less than",
            CountMatchingStrategy::LessThanOrEqual(_) => "less than or exactly",
            CountMatchingStrategy::EqualTo(_) => "exactly",
            CountMatchingStrategy::GreaterThanOrEqual(_) => "more than or exactly",
            CountMatchingStrategy::GreaterThan(_) => "more than",
        }
    }

    fn is_match(&self, actual: u32) -> bool {
        match *self {
            CountMatchingStrategy::LessThan(expected) => actual < expected,
            CountMatchingStrategy::LessThanOrEqual(expected) => actual <= expected,
            CountMatchingStrategy::EqualTo(expected) => actual == expected,
            CountMatchingStrategy::GreaterThanOrEqual(expected) => actual >= expected,
            CountMatchingStrategy::GreaterThan(expected) => actual > expected,
        }
    }
}

impl fmt::Display for CountMatchingStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.friendly_name(), self.expected())
    }
}

impl From<u8> for CountMatchingStrategy {
    fn from(expected: u8) -> Self {
        CountMatchingStrategy::EqualTo(expected.into())
    }
}

impl From<u16> for CountMatchingStrategy {
    fn from(expected: u16) -> Self {
        CountMatchingStrategy::EqualTo(expected.into())
    }
}

impl From<u32> for CountMatchingStrategy {
    fn from(expected: u32) -> Self {
        CountMatchingStrategy::EqualTo(expected)
    }
}

impl From<i8> for CountMatchingStrategy {
    fn from(expected: i8) -> Self {
        CountMatchingStrategy::EqualTo(expected.try_into().expect("expected u32, but number was negative"))
    }
}

impl From<i16> for CountMatchingStrategy {
    fn from(expected: i16) -> Self {
        CountMatchingStrategy::EqualTo(expected.try_into().expect("expected u32, but number was negative"))
    }
}

impl From<i32> for CountMatchingStrategy {
    fn from(expected: i32) -> Self {
        CountMatchingStrategy::EqualTo(expected.try_into().expect("expected u32, but number was negative"))
    }
}

impl From<i64> for CountMatchingStrategy {
    fn from(expected: i64) -> Self {
        CountMatchingStrategy::EqualTo(expected.try_into().expect("expected u32, but number was negative"))
    }
}
