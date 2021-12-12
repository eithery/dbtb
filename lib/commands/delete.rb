#
# Eithery, 2021
# CLI database delete command
# Deletes a database
#
module Dbtb
  class CLI
    desc 'delete DATABASE_NAME', 'Deletes a database'

    def delete(database_name)
      puts "DELETE DATABASE #{database_name}"
    end
  end
end
