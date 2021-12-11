#
# Eithery, 2021
# CLI database check command
# Compares the actual DB schema against openDB schema
#
module Dbtb
  class CLI
    desc 'check', 'Compare DB schema against the predefined openDB schema'

    command :check, :c do
      puts 'CHECK DATABASE!'
    end
  end
end
