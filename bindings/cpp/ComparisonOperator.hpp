#ifndef ComparisonOperator_HPP
#define ComparisonOperator_HPP

#include "ComparisonOperator.d.hpp"

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

inline diplomat::capi::ComparisonOperator ComparisonOperator::AsFFI() const {
  return static_cast<diplomat::capi::ComparisonOperator>(value);
}

inline ComparisonOperator ComparisonOperator::FromFFI(diplomat::capi::ComparisonOperator c_enum) {
  switch (c_enum) {
    case diplomat::capi::ComparisonOperator_Eq:
    case diplomat::capi::ComparisonOperator_Neq:
    case diplomat::capi::ComparisonOperator_Gt:
    case diplomat::capi::ComparisonOperator_Gte:
    case diplomat::capi::ComparisonOperator_Lt:
    case diplomat::capi::ComparisonOperator_Lte:
    case diplomat::capi::ComparisonOperator_In:
    case diplomat::capi::ComparisonOperator_NotIn:
    case diplomat::capi::ComparisonOperator_Contains:
    case diplomat::capi::ComparisonOperator_ContainsAll:
    case diplomat::capi::ComparisonOperator_ContainsAny:
    case diplomat::capi::ComparisonOperator_ArrayLengthEq:
    case diplomat::capi::ComparisonOperator_ArrayLengthGt:
    case diplomat::capi::ComparisonOperator_ArrayLengthLt:
      return static_cast<ComparisonOperator::Value>(c_enum);
    default:
      std::abort();
  }
}
#endif // ComparisonOperator_HPP
