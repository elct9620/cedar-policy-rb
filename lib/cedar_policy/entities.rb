# frozen_string_literal: true

module CedarPolicy
  # :nodoc:
  class Entities
    include Enumerable

    def initialize(entities = [])
      @entities = Set.new(entities.map do |entity|
        next entity if entity.is_a?(Entity)

        Entity.new(*entity.values_at(:uid, :attrs, :parents))
      end)
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
