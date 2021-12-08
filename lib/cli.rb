#
# Eithery, 2021
# DBTB CLI entry point
#
require_relative 'dbtb'

module Dbtb
  class CLI < Thor
    extend CommandHelper
    Dir["#{__dir__}/commands/**/*.rb"].each { |f| require f }
  end
end
