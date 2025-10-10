#ifndef SigningKey_D_HPP
#define SigningKey_D_HPP

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
namespace diplomat::capi { struct VerifyingKey; }
class VerifyingKey;


namespace diplomat {
namespace capi {
    struct SigningKey;
} // namespace capi
} // namespace

/**
 * Represents a signing key for Starknet transactions
 */
class SigningKey {
public:

  /**
   * Creates a new signing key from a secret scalar
   */
  inline static diplomat::result<std::unique_ptr<SigningKey>, std::unique_ptr<DojoError>> new_(std::string_view secret_scalar);

  /**
   * Generates a new random signing key
   */
  inline static std::unique_ptr<SigningKey> from_random();

  /**
   * Returns the secret scalar of the signing key
   */
  inline std::string secret_scalar() const;
  template<typename W>
  inline void secret_scalar_write(W& writeable_output) const;

  /**
   * Signs a message hash
   */
  inline diplomat::result<std::unique_ptr<Signature>, std::unique_ptr<DojoError>> sign(const FieldElement& hash) const;

  /**
   * Returns the verifying key
   */
  inline std::unique_ptr<VerifyingKey> verifying_key() const;

  inline const diplomat::capi::SigningKey* AsFFI() const;
  inline diplomat::capi::SigningKey* AsFFI();
  inline static const SigningKey* FromFFI(const diplomat::capi::SigningKey* ptr);
  inline static SigningKey* FromFFI(diplomat::capi::SigningKey* ptr);
  inline static void operator delete(void* ptr);
private:
  SigningKey() = delete;
  SigningKey(const SigningKey&) = delete;
  SigningKey(SigningKey&&) noexcept = delete;
  SigningKey operator=(const SigningKey&) = delete;
  SigningKey operator=(SigningKey&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // SigningKey_D_HPP
