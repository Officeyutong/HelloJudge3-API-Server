use serde::{Deserialize, Serialize};

use crate::declare_simple_json_type;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ProblemFileEntry {
    pub name: String,
    pub last_modified_time: Option<f64>,
    pub size: u64,
}

declare_simple_json_type!(ProblemFile, Vec::<ProblemFileEntry>, problem_file);
