use std::{path::PathBuf, sync::Arc};

use celery::{prelude::RedisBroker, Celery};
use sea_orm::DatabaseConnection;

use crate::config::Config;

use super::permission::PermissionManager;

pub struct HJ3State {
    pub config: Config,
    pub judge_queue: Celery<RedisBroker>,
    pub common_cache: redis::Client,
    pub phoneauth_cache: redis::Client,
    pub redis_lock: relock::Relock,
    pub db: Arc<DatabaseConnection>,
    pub version_str: String,
    pub base_dir: PathBuf,
    pub perm_manager: PermissionManager,
}
