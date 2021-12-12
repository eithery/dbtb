#
# Eithery, 2021
# CLI version command
# Displays the application version
#
module Dbtb
  class CLI
    desc 'version', 'Display the application version'
    map %w[-v --version] => :version

    def version
      puts APP_TITLE
    end
  end
end
