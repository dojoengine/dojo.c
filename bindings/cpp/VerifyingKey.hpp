#ifndef VerifyingKey_HPP
#define VerifyingKey_HPP

#include "VerifyingKey.d.hpp"

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
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {

    void VerifyingKey_scalar(const diplomat::capi::VerifyingKey* self, diplomat::capi::DiplomatWrite* write);

    typedef struct VerifyingKey_verify_result {union {bool ok; diplomat::capi::DojoError* err;}; bool is_ok;} VerifyingKey_verify_result;
    VerifyingKey_verify_result VerifyingKey_verify(const diplomat::capi::VerifyingKey* self, const diplomat::capi::FieldElement* hash, const diplomat::capi::Signature* signature);

    void VerifyingKey_destroy(VerifyingKey* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::string VerifyingKey::scalar() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::VerifyingKey_scalar(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void VerifyingKey::scalar_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::VerifyingKey_scalar(this->AsFFI(),
    &write);
}

inline diplomat::result<bool, std::unique_ptr<DojoError>> VerifyingKey::verify(const FieldElement& hash, const Signature& signature) const {
  auto result = diplomat::capi::VerifyingKey_verify(this->AsFFI(),
    hash.AsFFI(),
    signature.AsFFI());
  return result.is_ok ? diplomat::result<bool, std::unique_ptr<DojoError>>(diplomat::Ok<bool>(result.ok)) : diplomat::result<bool, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline const diplomat::capi::VerifyingKey* VerifyingKey::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::VerifyingKey*>(this);
}

inline diplomat::capi::VerifyingKey* VerifyingKey::AsFFI() {
  return reinterpret_cast<diplomat::capi::VerifyingKey*>(this);
}

inline const VerifyingKey* VerifyingKey::FromFFI(const diplomat::capi::VerifyingKey* ptr) {
  return reinterpret_cast<const VerifyingKey*>(ptr);
}

inline VerifyingKey* VerifyingKey::FromFFI(diplomat::capi::VerifyingKey* ptr) {
  return reinterpret_cast<VerifyingKey*>(ptr);
}

inline void VerifyingKey::operator delete(void* ptr) {
  diplomat::capi::VerifyingKey_destroy(reinterpret_cast<diplomat::capi::VerifyingKey*>(ptr));
}


#endif // VerifyingKey_HPP
