#ifndef TypedData_D_HPP
#define TypedData_D_HPP

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


namespace diplomat {
namespace capi {
    struct TypedData;
} // namespace capi
} // namespace

/**
 * Represents a typed data structure for EIP-712 style signing
 */
class TypedData {
public:

  /**
   * Creates a new TypedData from JSON string
   */
  inline static diplomat::result<std::unique_ptr<TypedData>, std::unique_ptr<DojoError>> new_from_json(std::string_view json);

  inline const diplomat::capi::TypedData* AsFFI() const;
  inline diplomat::capi::TypedData* AsFFI();
  inline static const TypedData* FromFFI(const diplomat::capi::TypedData* ptr);
  inline static TypedData* FromFFI(diplomat::capi::TypedData* ptr);
  inline static void operator delete(void* ptr);
private:
  TypedData() = delete;
  TypedData(const TypedData&) = delete;
  TypedData(TypedData&&) noexcept = delete;
  TypedData operator=(const TypedData&) = delete;
  TypedData operator=(TypedData&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // TypedData_D_HPP
