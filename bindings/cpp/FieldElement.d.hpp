#ifndef FieldElement_D_HPP
#define FieldElement_D_HPP

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
    struct FieldElement;
} // namespace capi
} // namespace

/**
 * Represents a Starknet field element (Felt)
 */
class FieldElement {
public:

  /**
   * Creates a new FieldElement from a hexadecimal string
   */
  inline static diplomat::result<std::unique_ptr<FieldElement>, std::unique_ptr<DojoError>> new_from_hex(std::string_view hex);

  /**
   * Creates a new FieldElement from big-endian bytes
   */
  inline static std::unique_ptr<FieldElement> new_from_bytes(diplomat::span<const uint8_t> bytes);

  /**
   * Returns the field element as a hexadecimal string
   */
  inline std::string to_hex() const;
  template<typename W>
  inline void to_hex_write(W& writeable_output) const;

  /**
   * Returns the field element as bytes (big-endian)
   */
  inline void to_bytes(diplomat::span<uint8_t> result) const;

  inline const diplomat::capi::FieldElement* AsFFI() const;
  inline diplomat::capi::FieldElement* AsFFI();
  inline static const FieldElement* FromFFI(const diplomat::capi::FieldElement* ptr);
  inline static FieldElement* FromFFI(diplomat::capi::FieldElement* ptr);
  inline static void operator delete(void* ptr);
private:
  FieldElement() = delete;
  FieldElement(const FieldElement&) = delete;
  FieldElement(FieldElement&&) noexcept = delete;
  FieldElement operator=(const FieldElement&) = delete;
  FieldElement operator=(FieldElement&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // FieldElement_D_HPP
