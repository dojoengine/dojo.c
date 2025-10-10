#ifndef EnumOption_HPP
#define EnumOption_HPP

#include "EnumOption.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {

    void EnumOption_name(const diplomat::capi::EnumOption* self, diplomat::capi::DiplomatWrite* write);

    void EnumOption_destroy(EnumOption* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::string EnumOption::name() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::EnumOption_name(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void EnumOption::name_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::EnumOption_name(this->AsFFI(),
    &write);
}

inline const diplomat::capi::EnumOption* EnumOption::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::EnumOption*>(this);
}

inline diplomat::capi::EnumOption* EnumOption::AsFFI() {
  return reinterpret_cast<diplomat::capi::EnumOption*>(this);
}

inline const EnumOption* EnumOption::FromFFI(const diplomat::capi::EnumOption* ptr) {
  return reinterpret_cast<const EnumOption*>(ptr);
}

inline EnumOption* EnumOption::FromFFI(diplomat::capi::EnumOption* ptr) {
  return reinterpret_cast<EnumOption*>(ptr);
}

inline void EnumOption::operator delete(void* ptr) {
  diplomat::capi::EnumOption_destroy(reinterpret_cast<diplomat::capi::EnumOption*>(ptr));
}


#endif // EnumOption_HPP
