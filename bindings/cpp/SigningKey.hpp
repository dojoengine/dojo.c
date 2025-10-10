#ifndef SigningKey_HPP
#define SigningKey_HPP

#include "SigningKey.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "DojoError.hpp"
#include "FieldElement.hpp"
#include "Signature.hpp"
#include "VerifyingKey.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {

    typedef struct SigningKey_new_result {union {diplomat::capi::SigningKey* ok; diplomat::capi::DojoError* err;}; bool is_ok;} SigningKey_new_result;
    SigningKey_new_result SigningKey_new(diplomat::capi::DiplomatStringView secret_scalar);

    diplomat::capi::SigningKey* SigningKey_from_random(void);

    void SigningKey_secret_scalar(const diplomat::capi::SigningKey* self, diplomat::capi::DiplomatWrite* write);

    typedef struct SigningKey_sign_result {union {diplomat::capi::Signature* ok; diplomat::capi::DojoError* err;}; bool is_ok;} SigningKey_sign_result;
    SigningKey_sign_result SigningKey_sign(const diplomat::capi::SigningKey* self, const diplomat::capi::FieldElement* hash);

    diplomat::capi::VerifyingKey* SigningKey_verifying_key(const diplomat::capi::SigningKey* self);

    void SigningKey_destroy(SigningKey* self);

    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<SigningKey>, std::unique_ptr<DojoError>> SigningKey::new_(std::string_view secret_scalar) {
  auto result = diplomat::capi::SigningKey_new({secret_scalar.data(), secret_scalar.size()});
  return result.is_ok ? diplomat::result<std::unique_ptr<SigningKey>, std::unique_ptr<DojoError>>(diplomat::Ok<std::unique_ptr<SigningKey>>(std::unique_ptr<SigningKey>(SigningKey::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<SigningKey>, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline std::unique_ptr<SigningKey> SigningKey::from_random() {
  auto result = diplomat::capi::SigningKey_from_random();
  return std::unique_ptr<SigningKey>(SigningKey::FromFFI(result));
}

inline std::string SigningKey::secret_scalar() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::SigningKey_secret_scalar(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void SigningKey::secret_scalar_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::SigningKey_secret_scalar(this->AsFFI(),
    &write);
}

inline diplomat::result<std::unique_ptr<Signature>, std::unique_ptr<DojoError>> SigningKey::sign(const FieldElement& hash) const {
  auto result = diplomat::capi::SigningKey_sign(this->AsFFI(),
    hash.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<Signature>, std::unique_ptr<DojoError>>(diplomat::Ok<std::unique_ptr<Signature>>(std::unique_ptr<Signature>(Signature::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<Signature>, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline std::unique_ptr<VerifyingKey> SigningKey::verifying_key() const {
  auto result = diplomat::capi::SigningKey_verifying_key(this->AsFFI());
  return std::unique_ptr<VerifyingKey>(VerifyingKey::FromFFI(result));
}

inline const diplomat::capi::SigningKey* SigningKey::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::SigningKey*>(this);
}

inline diplomat::capi::SigningKey* SigningKey::AsFFI() {
  return reinterpret_cast<diplomat::capi::SigningKey*>(this);
}

inline const SigningKey* SigningKey::FromFFI(const diplomat::capi::SigningKey* ptr) {
  return reinterpret_cast<const SigningKey*>(ptr);
}

inline SigningKey* SigningKey::FromFFI(diplomat::capi::SigningKey* ptr) {
  return reinterpret_cast<SigningKey*>(ptr);
}

inline void SigningKey::operator delete(void* ptr) {
  diplomat::capi::SigningKey_destroy(reinterpret_cast<diplomat::capi::SigningKey*>(ptr));
}


#endif // SigningKey_HPP
