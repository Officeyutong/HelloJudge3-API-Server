use serde::{Deserialize, Serialize};

use crate::declare_simple_json_type;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ExampleEntry {
    pub input: String,
    pub output: String,
}

declare_simple_json_type!(Example, Vec::<ExampleEntry>, problem_example);
