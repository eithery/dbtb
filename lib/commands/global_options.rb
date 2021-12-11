#
# Eithery, 2021
# Application global options
#
module Dbtb
  class CLI
    config_file = File.join(__dir__, '../../config/dbtb.yml')
    defaults = ::YAML.load_file(config_file)

    class_option :host, desc: 'Database host/server name',
      aliases: ['-s', '--server', '--host'], default: defaults[:host]

    class_option :user, desc: 'User name to connect to db server',
      aliases: ['-u', '--user'], default: defaults[:user]

    class_option :password, desc: 'Password used to connect to db server',
      aliases: ['-p', '--password'], default: defaults[:password]

    class_option :data, desc: 'Path to openDB schema root directory',
      aliases: ['-d', '--db-schema', '--data'], default: defaults[:db_schema]

    class_option :colored, desc: 'Use colored console formatter to output messages',
      type: :boolean, aliases: ['-c'], default: false
  end
end
