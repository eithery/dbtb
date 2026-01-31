pub(crate) mod db;

use std::path::PathBuf;
use clap::Subcommand;
use crate::help;


#[derive(Subcommand)]
#[command(help_template = help::DEFAULT_TEMPLATE)]
pub(crate) enum Commands {
    #[command(
        about = "Build database(s) based on SQL or YAML database definitions",
        help_template = help::BUILD_COMMAND_TEMPLATE
    )]
    Build {
        #[arg(value_name = "SOURCE_PATH")]
        source: PathBuf,

        #[arg(short, long, value_name = "DATABASE_NAME", help = "Database name")]
        database: Option<String>,

        #[arg(short, long, value_name = "USER_LOGIN", help = "Database user login")]
        user: Option<String>,

        #[arg(short, long, value_name = "PASSWORD", help = "Database password")]
        password: Option<String>,

        #[arg(short, long, help = "Build ALL databases based on definitions")]
        all: bool,

        #[arg(short, long, help = "Rebuild existing databases")]
        force: bool
    },

    #[command(about = "The set of database related subcommands")]
    Db {
        #[command(subcommand)]
        command: db::Commands
    }
}
