use serde::{de::Error, Deserialize, Deserializer, Serialize};

use crate::declare_simple_json_type;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ProblemTestcaseEntry {
    pub input: String,
    pub output: String,
    pub full_score: i64,
}
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ProblemSubtaskEntry {
    pub name: String,
    pub score: i64,
    pub testcases: Vec<ProblemTestcaseEntry>,
    #[serde(deserialize_with = "all_to_i64_deserialize")]
    pub time_limit: i64,
    #[serde(deserialize_with = "all_to_i64_deserialize")]
    pub memory_limit: i64,
    #[serde(default = "String::new")]
    pub comment: String,
    pub method: SubtaskJudgingMethod,
}
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum SubtaskJudgingMethod {
    #[serde(rename = "min")]
    Min,
    #[serde(rename = "sum")]
    Sum,
}

declare_simple_json_type!(ProblemSubtask, Vec::<ProblemSubtaskEntry>, problem_subtask);

fn all_to_i64_deserialize<'de, D>(des: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    use serde_json::Value;
    let v1 = serde_json::Value::deserialize(des)?;
    match v1 {
        Value::Number(v) => Ok(v.as_i64().ok_or(Error::custom("Expected integer"))?),
        Value::String(s) => Ok(i64::from_str_radix(&s, 10).map_err(Error::custom)?),
        _ => Err(Error::custom("Unexpected type")),
    }
}
