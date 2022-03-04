use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RedisConfig {
    pub judge_queue: String,
    pub common_cache: String,
    pub phoneauth_cache: String,
    pub lock: String,
}
impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            judge_queue: "redis://localhost:6379/1".to_string(),
            common_cache: "redis://localhost:6379/2".to_string(),
            phoneauth_cache: "redis://localhost:6379/4".to_string(),
            lock: "redis://localhost:6379/5".to_string(),
        }
    }
}
