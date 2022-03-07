use std::time::Duration;

use clap::ArgMatches;
use log::info;
use sea_orm::{ConnectOptions, Database};
use sqlx::MySqlPool;

use crate::{config::Config, core::ResultType};
use anyhow::anyhow;

use self::{
    challenge::import_challenge, contest::import_contest, discussion::import_discussion,
    file::import_file, other::import_other, permission_pack::import_permission_pack,
    preliminary::import_preliminary, problem::import_problem, problemset::import_problemset,
    submission::import_submission, tag::import_tag, team::import_team, user::import_user,
    wiki::import_wiki,
};

use super::{cmd_init::init_handle, config::CLIConfig};

mod challenge;
mod contest;
mod discussion;
mod file;
mod other;
mod permission_pack;
mod preliminary;
mod problem;
mod problemset;
mod submission;
mod tag;
mod team;
mod user;
mod wiki;
pub async fn import_handle(
    config: &Config,
    args: &ArgMatches,
    cli_config: &CLIConfig,
) -> ResultType<()> {
    let hj2_db = args
        .value_of("hj2-db")
        .unwrap_or(cli_config.default_hj2_url.as_str());
    let mut opt = ConnectOptions::new(config.common.database_uri.clone());
    opt.max_lifetime(Duration::from_secs(60 * 60 * 24));
    opt.sqlx_logging(config.common.debug);
    let db = Database::connect(opt)
        .await
        .map_err(|e| anyhow!("Failed to connect to database: {}", e))?;
    let hj2_pool = MySqlPool::connect(hj2_db)
        .await
        .map_err(|e| anyhow!("Failed to connect to hj2 db: {}", e))?;
    if cli_config.create_tables {
        init_handle(config, false).await?;
    }
    if cli_config.import_config.user {
        import_user(&db, &mut hj2_pool.acquire().await?)
            .await
            .map_err(|e| anyhow!("Failed to import table: user: {}", e))?;
    }
    if cli_config.import_config.problem {
        import_problem(&db, &hj2_pool)
            .await
            .map_err(|e| anyhow!("Failed to import table: problem: {}", e))?;
    }
    if cli_config.import_config.contest {
        import_contest(&db, &hj2_pool)
            .await
            .map_err(|e| anyhow!("Failed to import table: contest: {}", e))?;
    }
    if cli_config.import_config.problemset {
        import_problemset(&db, &hj2_pool)
            .await
            .map_err(|e| anyhow!("Failed to import table series problemset: {}", e))?;
    }
    if cli_config.import_config.file_storage {
        import_file(&db, &hj2_pool)
            .await
            .map_err(|e| anyhow!("Failed to import table series file_storage: {}", e))?;
    }
    if cli_config.import_config.team {
        import_team(&db, &hj2_pool)
            .await
            .map_err(|e| anyhow!("Failed to import table series team: {}", e))?;
    }
    if cli_config.import_config.submission {
        import_submission(&db, &hj2_pool)
            .await
            .map_err(|e| anyhow!("Failed to import table series submission: {}", e))?;
    }
    if cli_config.import_config.wiki {
        import_wiki(&db, &hj2_pool)
            .await
            .map_err(|e| anyhow!("Failed to import table series wiki: {}", e))?;
    }
    if cli_config.import_config.permission_pack {
        import_permission_pack(&db, &hj2_pool)
            .await
            .map_err(|e| anyhow!("Failed to import table series permission_pack: {}", e))?;
    }
    if cli_config.import_config.challenge {
        import_challenge(&db, &hj2_pool)
            .await
            .map_err(|e| anyhow!("Failed to import table series challenge: {}", e))?;
    }
    if cli_config.import_config.discussion {
        import_discussion(&db, &hj2_pool)
            .await
            .map_err(|e| anyhow!("Failed to import table series discussion: {}", e))?;
    }
    if cli_config.import_config.tag {
        import_tag(&db, &hj2_pool)
            .await
            .map_err(|e| anyhow!("Failed to import table series tag: {}", e))?;
    }
    if cli_config.import_config.preliminary {
        import_preliminary(&db, &hj2_pool)
            .await
            .map_err(|e| anyhow!("Failed to import table series preliminary: {}", e))?;
    }
    if cli_config.import_config.other {
        import_other(&db, &hj2_pool)
            .await
            .map_err(|e| anyhow!("Failed to import table series other: {}", e))?;
    }
    info!("Importing finished");
    return Ok(());
}
