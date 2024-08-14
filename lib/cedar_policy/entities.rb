# frozen_string_literal: true

module CedarPolicy
  # :nodoc:
  class Entities
    def initialize(entities = [])
      @entities = Set.new(entities)
    end

    def to_ary
      @entities.map { |entity| CedarPolicy.deep_serialize(entity) }
    end
  end
end
