#ifndef Clause_HPP
#define Clause_HPP

#include "Clause.d.hpp"

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

    void Clause_destroy(Clause* self);

    } // extern "C"
} // namespace capi
} // namespace

inline const diplomat::capi::Clause* Clause::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::Clause*>(this);
}

inline diplomat::capi::Clause* Clause::AsFFI() {
  return reinterpret_cast<diplomat::capi::Clause*>(this);
}

inline const Clause* Clause::FromFFI(const diplomat::capi::Clause* ptr) {
  return reinterpret_cast<const Clause*>(ptr);
}

inline Clause* Clause::FromFFI(diplomat::capi::Clause* ptr) {
  return reinterpret_cast<Clause*>(ptr);
}

inline void Clause::operator delete(void* ptr) {
  diplomat::capi::Clause_destroy(reinterpret_cast<diplomat::capi::Clause*>(ptr));
}


#endif // Clause_HPP
