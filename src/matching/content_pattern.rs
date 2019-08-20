use std::fmt::Debug;

use serde::{Deserialize, Serialize};

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
    // MatchesJsonPath,
    // MatchesXPath,
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
            absent: "(absent)".to_string(),
        }
    }
}

impl StringValuePattern for AbsentPattern {
    fn value(&self) -> &str {
        &self.absent
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
    pub fn new<S: Into<String>>(
        json: S,
        ignore_array_order: Option<bool>,
        ignore_extra_elements: Option<bool>,
    ) -> EqualToJsonPattern
    {
        EqualToJsonPattern {
            equal_to_json: json.into(),
            ignore_array_order,
            ignore_extra_elements,
        }
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
    pub fn new<S: Into<String>>(
        xml: S,
        enable_placeholders: Option<bool>,
        placeholder_opening_delimiter_regex: Option<String>,
        placeholder_closing_delimiter_regex: Option<String>,
    ) -> EqualToXmlPattern
    {
        EqualToXmlPattern {
            equal_to_xml: xml.into(),
            enable_placeholders,
            placeholder_opening_delimiter_regex,
            placeholder_closing_delimiter_regex,
        }
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
