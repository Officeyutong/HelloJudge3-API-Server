use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProblemFileEntry {
    pub problem_id: i32,
    pub file_name: String,
    pub file_id: String,
}
