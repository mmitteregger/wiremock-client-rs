use crate::matching::*;

pub fn equal_to<S>(value: S) -> EqualToPattern
    where S: Into<String>,
{
    EqualToPattern::new(value)
}

pub fn binary_equal_to<B>(content: B) -> BinaryEqualToPattern
    where B: Into<Vec<u8>>,
{
    BinaryEqualToPattern::new(content)
}

pub fn equal_to_ignore_case<S>(value: S) -> EqualToPattern
    where S: Into<String>,
{
    EqualToPattern::new(value).with_case_insensitive(true)
}

pub fn equal_to_json<S>(json: S) -> EqualToJsonPattern
    where S: Into<String>,
{
    EqualToJsonPattern::new(json)
}

pub fn matching_json_path<S>(json_path: S) -> MatchesJsonPathPattern
    where S: Into<String>,
{
    MatchesJsonPathPattern::new(json_path)
}

pub fn equal_to_xml<S>(xml: S) -> EqualToXmlPattern
    where S: Into<String>,
{
    EqualToXmlPattern::new(xml)
}

pub fn matching_xpath<S>(value: S) -> MatchesXPathPattern
    where S: Into<String>,
{
    MatchesXPathPattern::new(value)
}

pub fn containing<S>(value: S) -> ContainsPattern
    where S: Into<String>,
{
    ContainsPattern::new(value)
}

pub fn matching<S>(regex: S) -> RegexPattern
    where S: Into<String>,
{
    RegexPattern::new(regex)
}

pub fn not_matching<S>(regex: S) -> NegativeRegexPattern
    where S: Into<String>,
{
    NegativeRegexPattern::new(regex)
}

pub fn absent() -> AbsentPattern {
    AbsentPattern::new()
}
