use serde::{Deserialize, Serialize};

use crate::declare_simple_json_type;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ExtraParameterEntry {
    #[serde(rename = "lang")]
    pub lang_regexpr: String,
    pub parameter: String,
    pub force: bool,
}
declare_simple_json_type!(ExtraParameter, Vec::<ExtraParameterEntry>, extra_parameter);
