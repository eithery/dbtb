#
# Eithery, 2021
# CommandHelper module
# Contains helper methods for commands
#
module Dbtb
  module CommandHelper
    def command(name, *aliases, &block)
      help_option
      define_method(name) do
        if options[:help]
          invoke :help, [name]
        else
          block.call
        end
      end

      map Array(aliases).map(&:to_s) => name
    end
  end

  private

  def help_option
    method_option :help, aliases: '-h', type: :boolean, desc: 'Display usage information'
  end
end
