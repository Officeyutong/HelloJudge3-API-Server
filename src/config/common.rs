use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CommonConfig {
    pub session_key: String,
    pub database_uri: String,
    pub debug: bool,
    pub logging_level: String,
    pub upload_dir: String,
    pub port: u16,
    pub host: String,
    pub app_name: String,
    pub password_salt: String,
    pub username_regex: String,
    // pub redis_uri: String,
    // pub cache_url: String,
    // pub background
    // pub redis_phoneauth_uri: String,
    // pub redis_lock_uri: String,
    pub judgers: HashMap<String, String>,
    pub fail_submit_penalty: u32,
    pub ranklist_update_interval: u32,
    pub ranklist_update_interval_closed_contests: u32,
    pub accepted_problems_refresh_interval: u32,
    pub enable_csrf_token: bool,
    // pub judge_status: ,
    pub use_polling: bool,
    pub following_count_limit: i32,
    pub feed_stream_refresh_interval: u32,
    pub max_problemtodo_count: u32,
    pub gravatar_url_prefix: String,
    pub disable_ioi_contests: bool,
    pub rate_limit: Vec<String>,
}

impl Default for CommonConfig {
    fn default() -> Self {
        Self {
            session_key: "qwkqpoksqi0xoqwkqpoksqi0xoqwkqpoksqi0xoqwkqpoksqi0xo".to_string(),
            database_uri: "sqlite:///data.db".to_string(),
            debug: false,
            logging_level: "info".to_string(),
            upload_dir: "uploads".to_string(),
            port: 8080,
            host: "0.0.0.0".to_string(),
            app_name: "HelloJudge3".to_string(),
            password_salt: "qeiduew9odpjh8q9uohr8".to_string(),
            username_regex: r#"^[a-zA-Z0-9_-]+$"#.to_string(),
            // redis_uri: "redis://localhost:6379/1".to_string(),
            // cache_url: "redis://localhost:6379/2".to_string(),
            // redis_phoneauth_uri: "redis://localhost:6379/4".to_string(),
            // redis_lock_uri: "redis://localhost:6379/5".to_string(),
            judgers: HashMap::from([("UUID".to_string(), "评测机名".to_string())]),
            fail_submit_penalty: 20,
            ranklist_update_interval: 60,
            ranklist_update_interval_closed_contests: 60 * 30,
            accepted_problems_refresh_interval: 5 * 60,
            enable_csrf_token: false,
            // judge_status: ,
            use_polling: false,
            following_count_limit: 30,
            feed_stream_refresh_interval: 30,
            max_problemtodo_count: 50,
            gravatar_url_prefix: "https://www.gravatar.com/avatar/".to_string(),
            disable_ioi_contests: false,
            rate_limit: vec!["20 per second".to_string()],
        }
    }
}
