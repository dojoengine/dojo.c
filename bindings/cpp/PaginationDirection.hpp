#ifndef PaginationDirection_HPP
#define PaginationDirection_HPP

#include "PaginationDirection.d.hpp"

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

inline diplomat::capi::PaginationDirection PaginationDirection::AsFFI() const {
  return static_cast<diplomat::capi::PaginationDirection>(value);
}

inline PaginationDirection PaginationDirection::FromFFI(diplomat::capi::PaginationDirection c_enum) {
  switch (c_enum) {
    case diplomat::capi::PaginationDirection_Forward:
    case diplomat::capi::PaginationDirection_Backward:
      return static_cast<PaginationDirection::Value>(c_enum);
    default:
      std::abort();
  }
}
#endif // PaginationDirection_HPP
