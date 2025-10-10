#ifndef OrderDirection_HPP
#define OrderDirection_HPP

#include "OrderDirection.d.hpp"

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

inline diplomat::capi::OrderDirection OrderDirection::AsFFI() const {
  return static_cast<diplomat::capi::OrderDirection>(value);
}

inline OrderDirection OrderDirection::FromFFI(diplomat::capi::OrderDirection c_enum) {
  switch (c_enum) {
    case diplomat::capi::OrderDirection_Asc:
    case diplomat::capi::OrderDirection_Desc:
      return static_cast<OrderDirection::Value>(c_enum);
    default:
      std::abort();
  }
}
#endif // OrderDirection_HPP
