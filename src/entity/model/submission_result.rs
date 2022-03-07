use serde::{Deserialize, Serialize};

use crate::declare_simple_json_type;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct SubmissionSubtaskResult {
    pub score: i32,
    pub status: String,
    pub testcases: Vec<SubmissionTestcaseResult>,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct SubmissionTestcaseResult {
    pub full_score: i32,
    pub input: String,
    pub output: String,
    pub status: String,
    pub score: i32,
    pub message: String,
    pub time_cost: i64,
    pub memory_cost: i64,
}

declare_simple_json_type!(SubmissionResult, std::collections::HashMap<String,SubmissionSubtaskResult>, submission_result);
