#ifndef Signature_HPP
#define Signature_HPP

#include "Signature.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "FieldElement.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {

    diplomat::capi::Signature* Signature_new(const diplomat::capi::FieldElement* r, const diplomat::capi::FieldElement* s);

    void Signature_r(const diplomat::capi::Signature* self, diplomat::capi::DiplomatWrite* write);

    void Signature_s(const diplomat::capi::Signature* self, diplomat::capi::DiplomatWrite* write);

    void Signature_destroy(Signature* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<Signature> Signature::new_(const FieldElement& r, const FieldElement& s) {
  auto result = diplomat::capi::Signature_new(r.AsFFI(),
    s.AsFFI());
  return std::unique_ptr<Signature>(Signature::FromFFI(result));
}

inline std::string Signature::r() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::Signature_r(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void Signature::r_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::Signature_r(this->AsFFI(),
    &write);
}

inline std::string Signature::s() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::Signature_s(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void Signature::s_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::Signature_s(this->AsFFI(),
    &write);
}

inline const diplomat::capi::Signature* Signature::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::Signature*>(this);
}

inline diplomat::capi::Signature* Signature::AsFFI() {
  return reinterpret_cast<diplomat::capi::Signature*>(this);
}

inline const Signature* Signature::FromFFI(const diplomat::capi::Signature* ptr) {
  return reinterpret_cast<const Signature*>(ptr);
}

inline Signature* Signature::FromFFI(diplomat::capi::Signature* ptr) {
  return reinterpret_cast<Signature*>(ptr);
}

inline void Signature::operator delete(void* ptr) {
  diplomat::capi::Signature_destroy(reinterpret_cast<diplomat::capi::Signature*>(ptr));
}


#endif // Signature_HPP
