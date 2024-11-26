# frozen_string_literal: true

module CedarPolicy
  # :nodoc:
  class Entities
    include Enumerable

    # Include schema in Entities to enable Cedar to evaluate Action groups.
    attr_accessor :schema

    def initialize(entities = [], schema: nil)
      @entities = Set.new(entities.map do |entity|
        next entity if entity.is_a?(Entity)

        Entity.new(*entity.values_at(:uid, :attrs, :parents))
      end)

      if schema
        schema = Schema.new(schema) unless schema.is_a?(Schema)
        self.schema = schema
      end
    end

    def each(&block)
      return enum_for(:each) unless block_given?

      @entities.each(&block)
    end

    def to_ary
      @entities.map { |entity| CedarPolicy.deep_serialize(entity) }
    end
  end
end
