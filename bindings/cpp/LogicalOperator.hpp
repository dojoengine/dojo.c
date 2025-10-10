#ifndef LogicalOperator_HPP
#define LogicalOperator_HPP

#include "LogicalOperator.d.hpp"

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

inline diplomat::capi::LogicalOperator LogicalOperator::AsFFI() const {
  return static_cast<diplomat::capi::LogicalOperator>(value);
}

inline LogicalOperator LogicalOperator::FromFFI(diplomat::capi::LogicalOperator c_enum) {
  switch (c_enum) {
    case diplomat::capi::LogicalOperator_And:
    case diplomat::capi::LogicalOperator_Or:
      return static_cast<LogicalOperator::Value>(c_enum);
    default:
      std::abort();
  }
}
#endif // LogicalOperator_HPP
