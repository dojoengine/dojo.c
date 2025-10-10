#ifndef U256_HPP
#define U256_HPP

#include "U256.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "DojoError.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {

    typedef struct U256_new_from_hex_result {union {diplomat::capi::U256* ok; diplomat::capi::DojoError* err;}; bool is_ok;} U256_new_from_hex_result;
    U256_new_from_hex_result U256_new_from_hex(diplomat::capi::DiplomatStringView hex);

    typedef struct U256_new_from_bytes_result {union {diplomat::capi::U256* ok; diplomat::capi::DojoError* err;}; bool is_ok;} U256_new_from_bytes_result;
    U256_new_from_bytes_result U256_new_from_bytes(diplomat::capi::DiplomatU8View bytes);

    void U256_to_hex(const diplomat::capi::U256* self, diplomat::capi::DiplomatWrite* write);

    void U256_to_bytes(const diplomat::capi::U256* self, diplomat::capi::DiplomatU8ViewMut result);

    void U256_destroy(U256* self);

    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<U256>, std::unique_ptr<DojoError>> U256::new_from_hex(std::string_view hex) {
  auto result = diplomat::capi::U256_new_from_hex({hex.data(), hex.size()});
  return result.is_ok ? diplomat::result<std::unique_ptr<U256>, std::unique_ptr<DojoError>>(diplomat::Ok<std::unique_ptr<U256>>(std::unique_ptr<U256>(U256::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<U256>, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<U256>, std::unique_ptr<DojoError>> U256::new_from_bytes(diplomat::span<const uint8_t> bytes) {
  auto result = diplomat::capi::U256_new_from_bytes({bytes.data(), bytes.size()});
  return result.is_ok ? diplomat::result<std::unique_ptr<U256>, std::unique_ptr<DojoError>>(diplomat::Ok<std::unique_ptr<U256>>(std::unique_ptr<U256>(U256::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<U256>, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline std::string U256::to_hex() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::U256_to_hex(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void U256::to_hex_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::U256_to_hex(this->AsFFI(),
    &write);
}

inline void U256::to_bytes(diplomat::span<uint8_t> result) const {
  diplomat::capi::U256_to_bytes(this->AsFFI(),
    {result.data(), result.size()});
}

inline const diplomat::capi::U256* U256::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::U256*>(this);
}

inline diplomat::capi::U256* U256::AsFFI() {
  return reinterpret_cast<diplomat::capi::U256*>(this);
}

inline const U256* U256::FromFFI(const diplomat::capi::U256* ptr) {
  return reinterpret_cast<const U256*>(ptr);
}

inline U256* U256::FromFFI(diplomat::capi::U256* ptr) {
  return reinterpret_cast<U256*>(ptr);
}

inline void U256::operator delete(void* ptr) {
  diplomat::capi::U256_destroy(reinterpret_cast<diplomat::capi::U256*>(ptr));
}


#endif // U256_HPP
