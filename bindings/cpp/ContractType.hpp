#ifndef ContractType_HPP
#define ContractType_HPP

#include "ContractType.d.hpp"

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

inline diplomat::capi::ContractType ContractType::AsFFI() const {
  return static_cast<diplomat::capi::ContractType>(value);
}

inline ContractType ContractType::FromFFI(diplomat::capi::ContractType c_enum) {
  switch (c_enum) {
    case diplomat::capi::ContractType_WORLD:
    case diplomat::capi::ContractType_ERC20:
    case diplomat::capi::ContractType_ERC721:
    case diplomat::capi::ContractType_ERC1155:
    case diplomat::capi::ContractType_UDC:
    case diplomat::capi::ContractType_OTHER:
      return static_cast<ContractType::Value>(c_enum);
    default:
      std::abort();
  }
}
#endif // ContractType_HPP
