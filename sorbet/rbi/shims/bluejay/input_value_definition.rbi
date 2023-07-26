# typed: strict
# frozen_string_literal: true

module Bluejay
  class InputValueDefinition
    sig do
      params(
        name: String,
        type: InputType,
        description: T.nilable(String),
        directives: T::Array[Base::Directive],
        ruby_name: T.nilable(String),
        default_value: T.nilable(Object),
        visibility: T.nilable(Visibility),
      ).void
    end
    def initialize(name:, type:, description: nil, directives: [], ruby_name: nil, default_value: nil, visibility: nil)
    end

    sig { returns(String) }
    def name; end

    sig { returns(InputType) }
    def type; end

    sig { returns(String) }
    def ruby_name; end
  end
end
