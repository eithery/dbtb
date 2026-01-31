pub(crate) mod db;

use clap::Subcommand;
use crate::help;


#[derive(Subcommand)]
#[command(help_template = help::DEFAULT_TEMPLATE)]
pub(crate) enum Commands {
    #[command(about = "The set of database related subcommands")]
    Db {
        #[command(subcommand)]
        command: db::Commands
    }
}
