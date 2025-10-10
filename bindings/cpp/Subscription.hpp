#ifndef Subscription_HPP
#define Subscription_HPP

#include "Subscription.d.hpp"

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

    uint64_t Subscription_id(const diplomat::capi::Subscription* self);

    void Subscription_destroy(Subscription* self);

    } // extern "C"
} // namespace capi
} // namespace

inline uint64_t Subscription::id() const {
  auto result = diplomat::capi::Subscription_id(this->AsFFI());
  return result;
}

inline const diplomat::capi::Subscription* Subscription::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::Subscription*>(this);
}

inline diplomat::capi::Subscription* Subscription::AsFFI() {
  return reinterpret_cast<diplomat::capi::Subscription*>(this);
}

inline const Subscription* Subscription::FromFFI(const diplomat::capi::Subscription* ptr) {
  return reinterpret_cast<const Subscription*>(ptr);
}

inline Subscription* Subscription::FromFFI(diplomat::capi::Subscription* ptr) {
  return reinterpret_cast<Subscription*>(ptr);
}

inline void Subscription::operator delete(void* ptr) {
  diplomat::capi::Subscription_destroy(reinterpret_cast<diplomat::capi::Subscription*>(ptr));
}


#endif // Subscription_HPP
