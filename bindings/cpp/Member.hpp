#ifndef Member_HPP
#define Member_HPP

#include "Member.d.hpp"

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

    void Member_name(const diplomat::capi::Member* self, diplomat::capi::DiplomatWrite* write);

    bool Member_is_key(const diplomat::capi::Member* self);

    void Member_destroy(Member* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::string Member::name() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::Member_name(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void Member::name_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::Member_name(this->AsFFI(),
    &write);
}

inline bool Member::is_key() const {
  auto result = diplomat::capi::Member_is_key(this->AsFFI());
  return result;
}

inline const diplomat::capi::Member* Member::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::Member*>(this);
}

inline diplomat::capi::Member* Member::AsFFI() {
  return reinterpret_cast<diplomat::capi::Member*>(this);
}

inline const Member* Member::FromFFI(const diplomat::capi::Member* ptr) {
  return reinterpret_cast<const Member*>(ptr);
}

inline Member* Member::FromFFI(diplomat::capi::Member* ptr) {
  return reinterpret_cast<Member*>(ptr);
}

inline void Member::operator delete(void* ptr) {
  diplomat::capi::Member_destroy(reinterpret_cast<diplomat::capi::Member*>(ptr));
}


#endif // Member_HPP
