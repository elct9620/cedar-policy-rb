# frozen_string_literal: true

require "json"
require "set"

require_relative "cedar_policy/version"
require_relative "cedar_policy/entity_uid"
require_relative "cedar_policy/entity"
require_relative "cedar_policy/entities"
require_relative "cedar_policy/context"

begin
  RUBY_VERSION =~ /(\d+\.\d+)/
  require "cedar_policy/#{Regexp.last_match(1)}/cedar_policy"
rescue LoadError
  require "cedar_policy/cedar_policy"
end

# :nodoc:
module CedarPolicy
  class Error < StandardError; end

  def self.deep_serialize(input)
    input.to_hash.each_with_object({}) do |(key, value), output|
      output[key.to_sym] =
        case value
        when ->(h) { h.respond_to?(:to_hash) } then deep_serialize(value)
        when Array
          value.map { |item| item.respond_to?(:to_hash) ? deep_serialize(item) : item }
        else
          value
        end
    end
  end
end
