use crate::matching::CountMatchingStrategy;

pub fn less_than(expected: u32) -> CountMatchingStrategy {
    CountMatchingStrategy::LessThan(expected)
}

pub fn less_than_or_exactly(expected: u32) -> CountMatchingStrategy {
    CountMatchingStrategy::LessThanOrEqual(expected)
}

pub fn exactly(expected: u32) -> CountMatchingStrategy {
    CountMatchingStrategy::EqualTo(expected)
}

pub fn more_than_or_exactly(expected: u32) -> CountMatchingStrategy {
    CountMatchingStrategy::GreaterThanOrEqual(expected)
}

pub fn more_than(expected: u32) -> CountMatchingStrategy {
    CountMatchingStrategy::GreaterThan(expected)
}
