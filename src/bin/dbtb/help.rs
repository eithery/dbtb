use constcat::concat;


const HELP_PREFIX: &str = "\n{before-help}\n{about-with-newline}\n";
const DEFAULT_USAGE: &str = "USAGE: {usage}\n";
const CUSTOM_USAGE: &str = "USAGE: dbtb <COMMAND> [OPTIONS...]\n";
const BUILD_COMMAND_USAGE: &str = "USAGE: dbtb build <SOURCE_PATH> [OPTIONS...]\n";
const COMMANDS: &str = "\nCOMMANDS:\n{subcommands}\n";
const DEFAULT_OPTIONS: &str = "\nOPTIONS:\n{options}";
const CUSTOM_OPTIONS: &str = "
OPTIONS:
  -h, --help     Print help
  -V, --version  Print version
";


pub(crate) const DEFAULT_TEMPLATE: &str = concat!(HELP_PREFIX, CUSTOM_USAGE, COMMANDS, CUSTOM_OPTIONS);
pub(crate) const COMMAND_GROUP_TEMPLATE: &str = concat!(HELP_PREFIX, DEFAULT_USAGE, COMMANDS, DEFAULT_OPTIONS);
pub(crate) const COMMAND_TEMPLATE: &str = concat!(HELP_PREFIX, DEFAULT_USAGE, DEFAULT_OPTIONS);
pub(crate) const BUILD_COMMAND_TEMPLATE: &str = concat!(HELP_PREFIX, BUILD_COMMAND_USAGE, DEFAULT_OPTIONS);
