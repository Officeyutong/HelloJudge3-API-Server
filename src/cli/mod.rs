use clap::{Arg, Command};
use log::debug;

use crate::{
    cli::{
        cmd_import::import_handle, cmd_init::init_handle, cmd_reset::reset_handle,
        config::CLIConfig,
    },
    config::Config,
    core::ResultType,
};

mod cmd_import;
mod cmd_init;
mod cmd_reset;
pub mod config;
mod model;
pub async fn cli_entry(cfg: &Config) -> ResultType<bool> {
    if !std::path::Path::new("cli_config.yaml").exists() {
        tokio::fs::write(
            "cli_config.yaml",
            serde_yaml::to_string(&CLIConfig::default())?,
        )
        .await?;
    }
    let cli_config = serde_yaml::from_str(&tokio::fs::read_to_string("cli_config.yaml").await?)?;

    let cmd = Command::new(env!("CARGO_CRATE_NAME"))
        .subcommand(Command::new("init").about("Initialize database. Use URI from config.yaml"))
        .subcommand(
            Command::new("dangerously-reset-database")
                .about("Reset the database. Remove all the tables and rows.")
                .arg(
                    Arg::new("db")
                        .help("Override Database name fron configure")
                        .takes_value(true)
                        .required(false),
                ),
        )
        .subcommand(
            Command::new("import")
                .about("Import database from HJ2")
                .arg(
                    Arg::new("hj2-db")
                        .help("HJ2 database URI")
                        .takes_value(true)
                        .required(false),
                ),
        );
    let matched = cmd.get_matches();

    debug!("{:#?}", matched);
    if let Some((cmd, args)) = matched.subcommand() {
        match cmd {
            "init" => init_handle(cfg, false).await?,
            "import" => import_handle(cfg, args, &cli_config).await?,
            "dangerously-reset-database" => reset_handle(cfg, args, &cli_config).await?,
            _ => todo!(),
        };
        return Ok(true);
    }
    return Ok(false);
}
