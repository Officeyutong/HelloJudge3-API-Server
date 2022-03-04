use celery::{prelude::RedisBroker, Celery};
use sea_orm::DatabaseConnection;

use crate::config::Config;

pub struct HJ2State {
    pub config: Config,
    pub judge_queue: Celery<RedisBroker>,
    pub common_cache: redis::Client,
    pub phoneauth_cache: redis::Client,
    pub redis_lock: relock::Relock,
    pub conn: DatabaseConnection,
    pub version_str: String,
}
