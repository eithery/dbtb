#
# Eithery, 2021
# CLI help command
# Displays the application help info
#
module Dbtb
  class CLI
    def help(*)
      if ARGV.empty?
        puts [APP_TITLE, APP_USAGE, HELP_MSG].join("\n\n")
      elsif args.empty?
        with_greeting { super }
      else
        super
      end
    end

    private

    def with_greeting
      puts [APP_TITLE, APP_USAGE].join("\n\n"), "\n"
      result = yield
      puts HELP_COMMAND_MSG
      result
    end

    HELP_MSG = "Run 'dbtb help' or 'dbtb -h' for more information about supported commands and options"
    HELP_COMMAND_MSG = "Run 'dbtb help <COMMAND>' for more information about a specific command"
    APP_TITLE = "OpenDB database toolbox. Version #{VERSION}"
    APP_USAGE = 'usage: dbtb [<global options>] <COMMAND> [<args>] [<command options>]'
  end
end
