#
# Eithery, 2021
# DBTB CLI entry point
#
require_relative 'dbtb'

module Dbtb
  class CLI < Thor
    extend CommandHelper
    Dir["#{__dir__}/commands/**/*.rb"].each { |f| require f }

    def self.exit_on_failure?
      true
    end
  end
end
