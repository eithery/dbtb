#
# Eithery, 2021
# CLI database sync command
# Performs DB synchronization based on the openDB schema
#
module Dbtb
  class CLI
    desc 'sync', 'Perform DB synchronization based on the openDB schema'

    command :sync do
      puts 'SYNC DATABASE'
    end
  end
end
