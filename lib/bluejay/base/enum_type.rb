# typed: strict
# frozen_string_literal: true

module Bluejay
  module Base
    module EnumType
      extend(T::Sig)
      extend(T::Helpers)

      interface!

      module ClassMethods
        extend(T::Sig)
        extend(T::Helpers)

        abstract!

        requires_ancestor { Class }

        sig { abstract.returns(EnumTypeDefinition) }
        def definition; end
      end

      mixes_in_class_methods(ClassMethods)
    end
  end
end
