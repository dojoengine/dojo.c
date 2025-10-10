#ifndef FieldElement_HPP
#define FieldElement_HPP

#include "FieldElement.d.hpp"

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

    typedef struct FieldElement_new_from_hex_result {union {diplomat::capi::FieldElement* ok; diplomat::capi::DojoError* err;}; bool is_ok;} FieldElement_new_from_hex_result;
    FieldElement_new_from_hex_result FieldElement_new_from_hex(diplomat::capi::DiplomatStringView hex);

    diplomat::capi::FieldElement* FieldElement_new_from_bytes(diplomat::capi::DiplomatU8View bytes);

    void FieldElement_to_hex(const diplomat::capi::FieldElement* self, diplomat::capi::DiplomatWrite* write);

    void FieldElement_to_bytes(const diplomat::capi::FieldElement* self, diplomat::capi::DiplomatU8ViewMut result);

    void FieldElement_destroy(FieldElement* self);

    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<FieldElement>, std::unique_ptr<DojoError>> FieldElement::new_from_hex(std::string_view hex) {
  auto result = diplomat::capi::FieldElement_new_from_hex({hex.data(), hex.size()});
  return result.is_ok ? diplomat::result<std::unique_ptr<FieldElement>, std::unique_ptr<DojoError>>(diplomat::Ok<std::unique_ptr<FieldElement>>(std::unique_ptr<FieldElement>(FieldElement::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<FieldElement>, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline std::unique_ptr<FieldElement> FieldElement::new_from_bytes(diplomat::span<const uint8_t> bytes) {
  auto result = diplomat::capi::FieldElement_new_from_bytes({bytes.data(), bytes.size()});
  return std::unique_ptr<FieldElement>(FieldElement::FromFFI(result));
}

inline std::string FieldElement::to_hex() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::FieldElement_to_hex(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void FieldElement::to_hex_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::FieldElement_to_hex(this->AsFFI(),
    &write);
}

inline void FieldElement::to_bytes(diplomat::span<uint8_t> result) const {
  diplomat::capi::FieldElement_to_bytes(this->AsFFI(),
    {result.data(), result.size()});
}

inline const diplomat::capi::FieldElement* FieldElement::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::FieldElement*>(this);
}

inline diplomat::capi::FieldElement* FieldElement::AsFFI() {
  return reinterpret_cast<diplomat::capi::FieldElement*>(this);
}

inline const FieldElement* FieldElement::FromFFI(const diplomat::capi::FieldElement* ptr) {
  return reinterpret_cast<const FieldElement*>(ptr);
}

inline FieldElement* FieldElement::FromFFI(diplomat::capi::FieldElement* ptr) {
  return reinterpret_cast<FieldElement*>(ptr);
}

inline void FieldElement::operator delete(void* ptr) {
  diplomat::capi::FieldElement_destroy(reinterpret_cast<diplomat::capi::FieldElement*>(ptr));
}


#endif // FieldElement_HPP
