use clap::{Arg, Command};
use log::debug;

use crate::{cli::init_handle::init_handle, config::Config, core::ResultType};

mod init_handle;
pub async fn cli_entry(cfg: &Config) -> ResultType<bool> {
    let cmd = Command::new(env!("CARGO_CRATE_NAME"))
        .subcommand(Command::new("init").about("Initialize database. Use URI from config.yaml"))
        .subcommand(
            Command::new("import")
                .about("Import database from HJ2")
                .arg(
                    Arg::new("hj2-db")
                        .help("HJ2 database URI")
                        .takes_value(true)
                        .required(true),
                ),
        );
    let matched = cmd.get_matches();

    debug!("{:#?}", matched);
    if let Some((cmd, args)) = matched.subcommand() {
        match cmd {
            "init" => init_handle(cfg, args).await?,
            _ => todo!(),
        };
        return Ok(true);
    }
    return Ok(false);
}
