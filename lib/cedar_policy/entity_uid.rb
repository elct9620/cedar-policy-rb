# frozen_string_literal: true

module CedarPolicy
  # :nodoc:
  class EntityUid
    attr_reader :type_name, :id

    def initialize(type_name, id)
      @type_name = type_name.to_s
      @id = id.to_s
    end

    def to_str
      "#{@type_name}::#{@id.inspect}"
    end
    alias to_s to_str
    alias inspect to_str

    def to_hash
      { type: @type_name, id: @id }
    end
  end
end
