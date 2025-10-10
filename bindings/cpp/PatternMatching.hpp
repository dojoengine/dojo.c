#ifndef PatternMatching_HPP
#define PatternMatching_HPP

#include "PatternMatching.d.hpp"

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

    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::capi::PatternMatching PatternMatching::AsFFI() const {
  return static_cast<diplomat::capi::PatternMatching>(value);
}

inline PatternMatching PatternMatching::FromFFI(diplomat::capi::PatternMatching c_enum) {
  switch (c_enum) {
    case diplomat::capi::PatternMatching_FixedLen:
    case diplomat::capi::PatternMatching_VariableLen:
      return static_cast<PatternMatching::Value>(c_enum);
    default:
      std::abort();
  }
}
#endif // PatternMatching_HPP
