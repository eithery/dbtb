mod cli;
mod commands;
mod help;

use std::process;
use clap::{CommandFactory, Parser};
use dbtb::build_databases;
use dbtb::Result;
use crate::cli::{Cli, APP_TITLE};
use crate::commands::Commands;


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

        None => Ok(Cli::command().print_help()?)
    }
}
