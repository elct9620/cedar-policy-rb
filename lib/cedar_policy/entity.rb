# frozen_string_literal: true

module CedarPolicy
  # :nodoc:
  class Entity
    attr_reader :uid, :attrs, :parents

    def initialize(uid, attrs = {}, parents = [])
      raise ArgumentError unless uid.is_a?(EntityUid) || uid.is_a?(Hash)

      @uid = if uid.is_a?(EntityUid)
               uid
             else
               EntityUid.new(*uid.values_at(:type, :id))
             end
      @attrs = attrs
      @parents = Set.new(parents)
    end

    def eql?(other)
      hash == other.hash
    end

    def hash
      [self.class, @uid].hash
    end

    def to_hash
      {
        uid: @uid,
        attrs: @attrs,
        parents: @parents.to_a
      }
    end
  end
end
