#ifndef CallType_HPP
#define CallType_HPP

#include "CallType.d.hpp"

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

inline diplomat::capi::CallType CallType::AsFFI() const {
  return static_cast<diplomat::capi::CallType>(value);
}

inline CallType CallType::FromFFI(diplomat::capi::CallType c_enum) {
  switch (c_enum) {
    case diplomat::capi::CallType_Execute:
    case diplomat::capi::CallType_ExecuteFromOutside:
      return static_cast<CallType::Value>(c_enum);
    default:
      std::abort();
  }
}
#endif // CallType_HPP
