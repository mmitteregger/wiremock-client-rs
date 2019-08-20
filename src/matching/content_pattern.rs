use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use indexmap::IndexMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContentPattern {
    Absent(AbsentPattern),
    Anything(AnythingPattern),
    BinaryEqualTo(BinaryEqualToPattern),
    Contains(ContainsPattern),
    EqualToJson(EqualToJsonPattern),
    EqualTo(EqualToPattern),
    EqualToXml(EqualToXmlPattern),
    MatchesJsonPath(MatchesJsonPathPattern),
    MatchesXPath(MatchesXPathPattern),
    Regex(RegexPattern),
    NegativeRegex(NegativeRegexPattern),
}

pub trait StringValuePattern: Debug + Serialize + Deserialize<'static> + Into<ContentPattern> {
    fn value(&self) -> &str;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbsentPattern {
    absent: String,
}

impl AbsentPattern {
    pub fn new() -> AbsentPattern {
        AbsentPattern {
            absent: String::new(),
        }
    }
}

impl StringValuePattern for AbsentPattern {
    fn value(&self) -> &str {
        "(absent)"
    }
}

impl Into<ContentPattern> for AbsentPattern {
    fn into(self) -> ContentPattern {
        ContentPattern::Absent(self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnythingPattern {
    anything: String,
}

impl AnythingPattern {
    pub fn new() -> AnythingPattern {
        AnythingPattern {
            anything: "(always)".to_string(),
        }
    }
}

impl StringValuePattern for AnythingPattern {
    fn value(&self) -> &str {
        &self.anything
    }
}

impl Into<ContentPattern> for AnythingPattern {
    fn into(self) -> ContentPattern {
        ContentPattern::Anything(self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BinaryEqualToPattern {
    /// Base64 encoded string.
    #[serde(rename = "binaryEqualTo", with = "crate::serde::base64")]
    binary_equal_to: Vec<u8>,
}

impl BinaryEqualToPattern {
    pub fn new<B: Into<Vec<u8>>>(bytes: B) -> BinaryEqualToPattern {
        BinaryEqualToPattern {
            binary_equal_to: bytes.into(),
        }
    }
}

impl Into<ContentPattern> for BinaryEqualToPattern {
    fn into(self) -> ContentPattern {
        ContentPattern::BinaryEqualTo(self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContainsPattern {
    contains: String,
}

impl ContainsPattern {
    pub fn new<S: Into<String>>(expected: S) -> ContainsPattern {
        ContainsPattern {
            contains: expected.into(),
        }
    }
}

impl StringValuePattern for ContainsPattern {
    fn value(&self) -> &str {
        &self.contains
    }
}

impl Into<ContentPattern> for ContainsPattern {
    fn into(self) -> ContentPattern {
        ContentPattern::Contains(self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EqualToJsonPattern {
    #[serde(rename = "equalToJson")]
    equal_to_json: String,
    #[serde(rename = "ignoreArrayOrder", skip_serializing_if = "Option::is_none")]
    ignore_array_order: Option<bool>,
    #[serde(rename = "ignoreExtraElements", skip_serializing_if = "Option::is_none")]
    ignore_extra_elements: Option<bool>,
}

impl EqualToJsonPattern {
    pub fn new<S: Into<String>>(json: S) -> EqualToJsonPattern {
        EqualToJsonPattern {
            equal_to_json: json.into(),
            ignore_array_order: None,
            ignore_extra_elements: None,
        }
    }

    pub fn with_ignore_array_order(mut self, ignore_array_order: bool) -> EqualToJsonPattern {
        self.ignore_array_order = Some(ignore_array_order);
        self
    }

    pub fn with_ignore_extra_elements(mut self, ignore_extra_elements: bool) -> EqualToJsonPattern {
        self.ignore_extra_elements = Some(ignore_extra_elements);
        self
    }
}

impl StringValuePattern for EqualToJsonPattern {
    fn value(&self) -> &str {
        &self.equal_to_json
    }
}

impl Into<ContentPattern> for EqualToJsonPattern {
    fn into(self) -> ContentPattern {
        ContentPattern::EqualToJson(self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EqualToPattern {
    #[serde(rename = "equalTo")]
    equal_to: String,
    #[serde(rename = "caseInsensitive", skip_serializing_if = "Option::is_none")]
    case_insensitive: Option<bool>,
}

impl EqualToPattern {
    pub fn new<S: Into<String>>(expected: S, case_insensitive: Option<bool>) -> EqualToPattern {
        EqualToPattern {
            equal_to: expected.into(),
            case_insensitive,
        }
    }

    pub fn with_case_insensitive(mut self, case_insensitive: bool) -> EqualToPattern {
        self.case_insensitive = Some(case_insensitive);
        self
    }
}

impl StringValuePattern for EqualToPattern {
    fn value(&self) -> &str {
        &self.equal_to
    }
}

impl Into<ContentPattern> for EqualToPattern {
    fn into(self) -> ContentPattern {
        ContentPattern::EqualTo(self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EqualToXmlPattern {
    #[serde(rename = "equalToXml")]
    equal_to_xml: String,
    #[serde(rename = "enablePlaceholders", skip_serializing_if = "Option::is_none")]
    enable_placeholders: Option<bool>,
    #[serde(rename = "placeholderOpeningDelimiterRegex", skip_serializing_if = "Option::is_none")]
    placeholder_opening_delimiter_regex: Option<String>,
    #[serde(rename = "placeholderClosingDelimiterRegex", skip_serializing_if = "Option::is_none")]
    placeholder_closing_delimiter_regex: Option<String>,
}

impl EqualToXmlPattern {
    pub fn new<S: Into<String>>(xml: S) -> EqualToXmlPattern {
        EqualToXmlPattern {
            equal_to_xml: xml.into(),
            enable_placeholders: None,
            placeholder_opening_delimiter_regex: None,
            placeholder_closing_delimiter_regex: None,
        }
    }

    pub fn with_enable_placeholders(mut self, enable_placeholders: bool) -> EqualToXmlPattern {
        self.enable_placeholders = Some(enable_placeholders);
        self
    }

    pub fn with_placeholder_delimiter_regexes<S>(mut self, opening_regex: S, closing_regex: S) -> EqualToXmlPattern
        where S: Into<String>,
    {
        self.placeholder_opening_delimiter_regex = Some(opening_regex.into());
        self.placeholder_closing_delimiter_regex = Some(closing_regex.into());
        self
    }
}

impl StringValuePattern for EqualToXmlPattern {
    fn value(&self) -> &str {
        &self.equal_to_xml
    }
}

impl Into<ContentPattern> for EqualToXmlPattern {
    fn into(self) -> ContentPattern {
        ContentPattern::EqualToXml(self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchesJsonPathPattern {
    #[serde(rename = "matchesJsonPath")]
    matches_json_path: String,
}

impl MatchesJsonPathPattern {
    pub fn new<S: Into<String>>(json_path: S) -> MatchesJsonPathPattern {
        MatchesJsonPathPattern {
            matches_json_path: json_path.into(),
        }
    }
}

impl StringValuePattern for MatchesJsonPathPattern {
    fn value(&self) -> &str {
        &self.matches_json_path
    }
}

impl Into<ContentPattern> for MatchesJsonPathPattern {
    fn into(self) -> ContentPattern {
        ContentPattern::MatchesJsonPath(self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchesXPathPattern {
    #[serde(rename = "matchesXPath")]
    matches_xpath: String,
    #[serde(rename = "xPathNamespaces")]
    namespaces: IndexMap<String, String>,
}

impl MatchesXPathPattern {
    pub fn new<S: Into<String>>(
        xpath: S,
        namespaces: IndexMap<String, String>,
    ) -> MatchesXPathPattern {
        MatchesXPathPattern {
            matches_xpath: xpath.into(),
            namespaces,
        }
    }

    pub fn with_xpath_namespace<S>(mut self, name: S, namespace_uri: S) -> MatchesXPathPattern
        where S: Into<String>
    {
        self.namespaces.insert(name.into(), namespace_uri.into());
        self
    }
}

impl StringValuePattern for MatchesXPathPattern {
    fn value(&self) -> &str {
        &self.matches_xpath
    }
}

impl Into<ContentPattern> for MatchesXPathPattern {
    fn into(self) -> ContentPattern {
        ContentPattern::MatchesXPath(self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegexPattern {
    matches: String,
}

impl RegexPattern {
    pub fn new<S: Into<String>>(regex: S) -> RegexPattern {
        RegexPattern {
            matches: regex.into(),
        }
    }
}

impl StringValuePattern for RegexPattern {
    fn value(&self) -> &str {
        &self.matches
    }
}

impl Into<ContentPattern> for RegexPattern {
    fn into(self) -> ContentPattern {
        ContentPattern::Regex(self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NegativeRegexPattern {
    #[serde(rename = "doesNotMatch")]
    does_not_match: String,
}

impl NegativeRegexPattern {
    pub fn new<S: Into<String>>(regex: S) -> NegativeRegexPattern {
        NegativeRegexPattern {
            does_not_match: regex.into(),
        }
    }
}

impl StringValuePattern for NegativeRegexPattern {
    fn value(&self) -> &str {
        &self.does_not_match
    }
}

impl Into<ContentPattern> for NegativeRegexPattern {
    fn into(self) -> ContentPattern {
        ContentPattern::NegativeRegex(self)
    }
}
