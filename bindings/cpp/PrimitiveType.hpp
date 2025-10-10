#ifndef PrimitiveType_HPP
#define PrimitiveType_HPP

#include "PrimitiveType.d.hpp"

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

inline diplomat::capi::PrimitiveType PrimitiveType::AsFFI() const {
  return static_cast<diplomat::capi::PrimitiveType>(value);
}

inline PrimitiveType PrimitiveType::FromFFI(diplomat::capi::PrimitiveType c_enum) {
  switch (c_enum) {
    case diplomat::capi::PrimitiveType_I8:
    case diplomat::capi::PrimitiveType_I16:
    case diplomat::capi::PrimitiveType_I32:
    case diplomat::capi::PrimitiveType_I64:
    case diplomat::capi::PrimitiveType_I128:
    case diplomat::capi::PrimitiveType_U8:
    case diplomat::capi::PrimitiveType_U16:
    case diplomat::capi::PrimitiveType_U32:
    case diplomat::capi::PrimitiveType_U64:
    case diplomat::capi::PrimitiveType_U128:
    case diplomat::capi::PrimitiveType_U256:
    case diplomat::capi::PrimitiveType_Bool:
    case diplomat::capi::PrimitiveType_Felt252:
    case diplomat::capi::PrimitiveType_ClassHash:
    case diplomat::capi::PrimitiveType_ContractAddress:
    case diplomat::capi::PrimitiveType_EthAddress:
      return static_cast<PrimitiveType::Value>(c_enum);
    default:
      std::abort();
  }
}
#endif // PrimitiveType_HPP
