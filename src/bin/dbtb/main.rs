mod cli;
mod commands;
mod help;

use std::process;
use clap::{CommandFactory, Parser};
use dbtb::{build_databases, create_database, create_tables, drop_database, drop_tables};
use dbtb::Result;
use crate::cli::{Cli, APP_TITLE};
use crate::commands::{Commands, db};


fn main() {
    if let Err(err) = run() {
        dbtb::cli::error(format!("{err}").as_str());
        process::exit(1);
    }
}


fn run() -> Result<()> {
    let cli = Cli::parse();
    if cli.version {
        println!("\n{APP_TITLE}\n");
        return Ok(());
    }
    match &cli.command {
        Some(Commands::Build {
            source,
            database,
            user,
            password,
            all,
            force
        }) => build_databases(source, database.as_deref(), user.as_deref(), password.as_deref(), *all, *force),

        Some(Commands::Db { command }) =>
            match command {
                db::Commands::Migrate {
                    template,
                    config,
                    force
                } => create_tables(template, config.as_deref(), *force),

                db::Commands::Rollback { template, config } => drop_tables(template, config.as_deref()),
                db::Commands::Create { config} => create_database(config.as_deref()),
                db::Commands::Drop { config } => drop_database(config.as_deref())
            }
        None => Ok(Cli::command().print_help()?)
    }
}
