use crate::cli::cli_entry;
use crate::{core::state::HJ2State, test::my_test};

use crate::core::ResultType;

use crate::config::Config;
use anyhow::anyhow;
use celery::{broker::RedisBrokerBuilder, CeleryBuilder};
use flexi_logger::{DeferredNow, Record, TS_DASHES_BLANK_COLONS_DOT_BLANK};
use log::info;
use sea_orm::{ConnectOptions, Database};
pub mod cli;
pub mod config;
pub mod core;
pub mod entity;
mod test;
pub fn my_log_format(
    w: &mut dyn std::io::Write,
    now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    write!(
        w,
        "[{}] {} [{}:{}] {}",
        now.format(TS_DASHES_BLANK_COLONS_DOT_BLANK),
        record.level(),
        record.module_path().unwrap_or("<unnamed>"),
        record.line().unwrap_or(0),
        &record.args()
    )
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> ResultType<()> {
    if !std::path::Path::new("config.yaml").exists() {
        tokio::fs::write(
            "config.yaml",
            serde_yaml::to_string(&Config::default()).unwrap(),
        )
        .await?;
        return Err(anyhow!(
            "Config not found. Default config file created, modify it and restart."
        ));
    }
    let config = match serde_yaml::from_str::<Config>(
        &tokio::fs::read_to_string("config.yaml")
            .await
            .map_err(|e| anyhow!("Failed to read config.yaml: {}", e))?,
    ) {
        Ok(v) => v,
        Err(e) => {
            tokio::fs::write(
                "config.default.yaml",
                serde_yaml::to_string(&Config::default()).unwrap(),
            )
            .await
            .unwrap();
            return Err(anyhow!(
                "Failed to load config: {}\nDefault config has been saved as config.default.yaml",
                e
            ));
        }
    };

    use flexi_logger::{Duplicate, FileSpec, Logger};
    Logger::try_with_str(&config.common.logging_level)
        .map_err(|_| anyhow!("Invalid loggine level: {}", config.common.logging_level))?
        .format(my_log_format)
        .log_to_file(
            FileSpec::default()
                .directory("logs")
                .basename("hj3-api-server"),
        )
        .duplicate_to_stdout(Duplicate::All)
        .start()
        .map_err(|e| anyhow!("Failed to start logger!\n{}", e))?;
    if cli_entry(&config).await? {
        return Ok(());
    }
    let mut opt = ConnectOptions::new(config.common.database_uri.clone());
    opt.sqlx_logging(config.common.debug);
    let db_conn = Database::connect(opt)
        .await
        .map_err(|e| anyhow!("Failed to connect to database: {}", e))?;
    let celery_app = CeleryBuilder::<RedisBrokerBuilder>::new(
        "hj3-api-server",
        config.redis.judge_queue.as_str(),
    )
    .build()
    .await
    .map_err(|e| {
        anyhow!(
            "Failed to build celery app: {:?}, {:?}",
            e,
            config.redis.judge_queue
        )
    })?;

    let common_cache = redis::Client::open(config.redis.common_cache.as_str()).map_err(|e| {
        anyhow!(
            "Failed to create common cache client: {}, {:?}",
            e,
            config.redis.common_cache.as_str()
        )
    })?;
    let phoneauth_cache =
        redis::Client::open(config.redis.phoneauth_cache.as_str()).map_err(|e| {
            anyhow!(
                "Failed to create phoneauth cache client: {}, {:?}",
                e,
                config.redis.phoneauth_cache.as_str()
            )
        })?;
    let redis_lock = relock::Relock::new(config.redis.lock.as_str()).map_err(|e| {
        anyhow!(
            "Failed to create lock client: {}, {:?}",
            e,
            config.redis.lock.as_str()
        )
    })?;
    common_cache.get_async_connection().await.map_err(|e| {
        anyhow!(
            "Failed to connect to common_cache: {}, {:?}",
            e,
            config.redis.common_cache
        )
    })?;
    phoneauth_cache.get_async_connection().await.map_err(|e| {
        anyhow!(
            "Failed to connect to phoneauth_cache: {}, {:?}",
            e,
            config.redis.lock
        )
    })?;
    let app_state = HJ2State {
        config,
        judge_queue: celery_app,
        // judge_queue: todo!(),
        common_cache,
        phoneauth_cache,
        redis_lock,
        conn: db_conn,
        version_str: format!("HelloJudge3 API Server {}", env!("CARGO_PKG_VERSION")),
    };
    info!("Starting: {}", app_state.version_str);
    my_test(&app_state).await?;
    return Ok(());
}
