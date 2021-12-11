#
# Eithery, 2021
# CLI database create command
# Creates a new database
#
module Dbtb
  class CLI
    desc 'create DATABASE_NAME', 'Create a new database based on the openDB schema'

    def create(database_name)
      puts "CREATE DATABASE #{database_name}"
    end
  end
end
