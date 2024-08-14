# frozen_string_literal: true

module CedarPolicy
  # :nodoc:
  class Context
    def initialize(context = {})
      @context = context
    end

    def to_hash
      CedarPolicy.deep_serialize(@context)
    end
  end
end
