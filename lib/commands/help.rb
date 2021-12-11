#
# Eithery, 2021
# CLI help command
# Displays the application help info
#
module Dbtb
  class CLI
    def help(*)
      args.empty? ? with_greeting { super } : super
    end

    private

    def with_greeting
      puts GREETING + $/
      result = yield
      puts HELP_COMMAND_MSG
      result
    end

    HELP_COMMAND_MSG = "Run 'dbtb help <COMMAND>' for more information about a specific command"
    GREETING = <<~EOS
      OpenDB Database toolbox for MS SQL Server. Version #{VERSION}

      usage: dbtb [<global options>] <COMMAND> [<args>] [<command options>]
    EOS
  end
end
