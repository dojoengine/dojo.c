#ifndef Primitive_D_HPP
#define Primitive_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"

namespace diplomat::capi { struct DojoError; }
class DojoError;
class PrimitiveType;


namespace diplomat {
namespace capi {
    struct Primitive;
} // namespace capi
} // namespace

/**
 * Represents a primitive Cairo type with its value
 */
class Primitive {
public:

  /**
   * Gets the primitive type
   */
  inline PrimitiveType primitive_type() const;

  /**
   * Creates a primitive from JSON
   */
  inline static diplomat::result<std::unique_ptr<Primitive>, std::unique_ptr<DojoError>> from_json(std::string_view json);

  /**
   * Serializes the primitive to JSON
   */
  inline diplomat::result<std::string, std::unique_ptr<DojoError>> to_json() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> to_json_write(W& writeable_output) const;

  inline const diplomat::capi::Primitive* AsFFI() const;
  inline diplomat::capi::Primitive* AsFFI();
  inline static const Primitive* FromFFI(const diplomat::capi::Primitive* ptr);
  inline static Primitive* FromFFI(diplomat::capi::Primitive* ptr);
  inline static void operator delete(void* ptr);
private:
  Primitive() = delete;
  Primitive(const Primitive&) = delete;
  Primitive(Primitive&&) noexcept = delete;
  Primitive operator=(const Primitive&) = delete;
  Primitive operator=(Primitive&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // Primitive_D_HPP
