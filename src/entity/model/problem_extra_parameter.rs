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
impl Default for ExtraParameter {
    fn default() -> Self {
        Self(
            serde_json::from_value(serde_json::json!([
                {"lang": "cpp", "parameter": "-std=c++98", "name": "C++98", "force": false},
                {"lang": "cpp", "parameter": "-std=c++11", "name": "C++11", "force": false},
                {"lang": "cpp", "parameter": "-std=c++14", "name": "C++14", "force": false},
                {"lang": "cpp", "parameter": "-std=c++17", "name": "C++17", "force": false},
                {"lang": ".*", "parameter": "-O2", "name": "O2优化", "force": false},
            ]))
            .unwrap(),
        )
    }
}
