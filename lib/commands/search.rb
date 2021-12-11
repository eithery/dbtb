#
# Eithery, 2021
# CLI database search command
# Performs search for the specified DB object in the actual DB
#
module Dbtb
  class CLI
    desc 'search', 'Perform search for the specified DB object'

    command :search, :s do
      puts 'SEARCH DB OBJECT'
    end
  end
end
