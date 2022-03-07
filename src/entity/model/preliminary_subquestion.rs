use serde::{de::Error, Deserialize, Serialize};

use crate::declare_simple_json_type;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ChoicesQuestion {
    pub choices: Vec<String>,
    pub answers: Vec<String>,
    pub score: f64,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct FillingBlankQuestion {
    pub score: f64,
    pub answers: Vec<String>,
    pub multiline: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PreliminarySubquestion {
    Choice(ChoicesQuestion),
    FillingBlank(FillingBlankQuestion),
}
impl Serialize for PreliminarySubquestion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            PreliminarySubquestion::Choice(v) => v.serialize(serializer),
            PreliminarySubquestion::FillingBlank(v) => v.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for PreliminarySubquestion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let dec = serde_json::Value::deserialize(deserializer)?;
        if dec.pointer("/multiline").is_some() {
            return Ok(Self::FillingBlank(
                serde_json::from_value(dec).map_err(Error::custom)?,
            ));
        } else {
            return Ok(Self::Choice(
                serde_json::from_value(dec).map_err(Error::custom)?,
            ));
        };
    }
}

declare_simple_json_type!(
    PreliminarySubquestionList,
    Vec::<PreliminarySubquestion>,
    preliminary_subquestion
);
