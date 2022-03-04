use serde::{Deserialize, Serialize};

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
    pub time_limit: i64,
    pub memory_limit: i64,
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
