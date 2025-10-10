#ifndef U256_D_HPP
#define U256_D_HPP

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
    struct U256;
} // namespace capi
} // namespace

/**
 * Represents a 256-bit unsigned integer
 */
class U256 {
public:

  /**
   * Creates a new U256 from a hexadecimal string
   */
  inline static diplomat::result<std::unique_ptr<U256>, std::unique_ptr<DojoError>> new_from_hex(std::string_view hex);

  /**
   * Creates a new U256 from big-endian bytes
   */
  inline static diplomat::result<std::unique_ptr<U256>, std::unique_ptr<DojoError>> new_from_bytes(diplomat::span<const uint8_t> bytes);

  /**
   * Returns the U256 as a hexadecimal string
   */
  inline std::string to_hex() const;
  template<typename W>
  inline void to_hex_write(W& writeable_output) const;

  /**
   * Returns the U256 as bytes (big-endian)
   */
  inline void to_bytes(diplomat::span<uint8_t> result) const;

  inline const diplomat::capi::U256* AsFFI() const;
  inline diplomat::capi::U256* AsFFI();
  inline static const U256* FromFFI(const diplomat::capi::U256* ptr);
  inline static U256* FromFFI(diplomat::capi::U256* ptr);
  inline static void operator delete(void* ptr);
private:
  U256() = delete;
  U256(const U256&) = delete;
  U256(U256&&) noexcept = delete;
  U256 operator=(const U256&) = delete;
  U256 operator=(U256&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // U256_D_HPP
