use clap::ArgMatches;
use futures::TryStreamExt;
use log::info;
use sqlx::{MySqlPool, Row};

use crate::{config::Config, core::ResultType};

use super::config::CLIConfig;
pub async fn reset_handle(
    cfg: &Config,
    args: &ArgMatches,
    cli_config: &CLIConfig,
) -> ResultType<()> {
    let db_pool = MySqlPool::connect(&cfg.common.database_uri).await?;
    let mut conn = db_pool.acquire().await?;
    let db_name = args
        .value_of("db")
        .unwrap_or(cli_config.default_reset_db_name.as_str());
    let mut table_names: Vec<String> = vec![];
    {
        let mut rows = sqlx::query(
            "SELECT DISTINCT TABLE_NAME FROM information_schema.COLUMNS WHERE TABLE_SCHEMA = ?",
        )
        .bind(db_name)
        .fetch(&mut conn);
        while let Some(row) = rows.try_next().await? {
            table_names.push(row.try_get("TABLE_NAME")?);
        }
    }
    {
        sqlx::query("SET FOREIGN_KEY_CHECKS = 0")
            .execute(&mut conn)
            .await?;
    }
    info!("Tables to drop: {:?}", table_names);
    for table in table_names.iter() {
        sqlx::query(&format!("DROP TABLE {}", table))
            .execute(&mut conn)
            .await?;
        info!("Table dropped: {}", table);
    }
    {
        sqlx::query("SET FOREIGN_KEY_CHECKS = 1")
            .execute(&mut conn)
            .await?;
    }
    info!("Database cleared.");
    return Ok(());
}
