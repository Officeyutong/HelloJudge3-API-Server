use serde::{Deserialize, Serialize};

use crate::declare_simple_json_type;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ForeignProblemEntry {
    pub name: String,
    pub url: String,
}

declare_simple_json_type!(
    ProblemsetForeignProblem,
    Vec::<ForeignProblemEntry>,
    problemset_foreign_problem
);
