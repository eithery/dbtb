use std::path::PathBuf;
use clap::Subcommand;
use crate::help;


#[derive(Subcommand)]
#[command(help_template = help::DEFAULT_TEMPLATE)]
pub(crate) enum Commands {
    #[command(
        about = "Build database(s) based on SQL or YAML database definitions",
        help_template = help::COMMAND_TEMPLATE
    )]
    Build {
        #[arg(short, long, value_name = "SOURCE_PATH", help = "Path to SQL or YAML database definitions")]
        source: Option<PathBuf>,

        #[arg(long, value_name = "DB_SERVER_HOST", help = "Database server host")]
        host: Option<String>,

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
    }
}
