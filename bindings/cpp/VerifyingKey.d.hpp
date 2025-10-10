#ifndef VerifyingKey_D_HPP
#define VerifyingKey_D_HPP

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
namespace diplomat::capi { struct FieldElement; }
class FieldElement;
namespace diplomat::capi { struct Signature; }
class Signature;


namespace diplomat {
namespace capi {
    struct VerifyingKey;
} // namespace capi
} // namespace

/**
 * Represents a verifying key for signature verification
 */
class VerifyingKey {
public:

  /**
   * Returns the scalar value of the verifying key
   */
  inline std::string scalar() const;
  template<typename W>
  inline void scalar_write(W& writeable_output) const;

  /**
   * Verifies a signature
   */
  inline diplomat::result<bool, std::unique_ptr<DojoError>> verify(const FieldElement& hash, const Signature& signature) const;

  inline const diplomat::capi::VerifyingKey* AsFFI() const;
  inline diplomat::capi::VerifyingKey* AsFFI();
  inline static const VerifyingKey* FromFFI(const diplomat::capi::VerifyingKey* ptr);
  inline static VerifyingKey* FromFFI(diplomat::capi::VerifyingKey* ptr);
  inline static void operator delete(void* ptr);
private:
  VerifyingKey() = delete;
  VerifyingKey(const VerifyingKey&) = delete;
  VerifyingKey(VerifyingKey&&) noexcept = delete;
  VerifyingKey operator=(const VerifyingKey&) = delete;
  VerifyingKey operator=(VerifyingKey&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // VerifyingKey_D_HPP
