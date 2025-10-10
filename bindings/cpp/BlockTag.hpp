#ifndef BlockTag_HPP
#define BlockTag_HPP

#include "BlockTag.d.hpp"

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

inline diplomat::capi::BlockTag BlockTag::AsFFI() const {
  return static_cast<diplomat::capi::BlockTag>(value);
}

inline BlockTag BlockTag::FromFFI(diplomat::capi::BlockTag c_enum) {
  switch (c_enum) {
    case diplomat::capi::BlockTag_Latest:
    case diplomat::capi::BlockTag_PreConfirmed:
      return static_cast<BlockTag::Value>(c_enum);
    default:
      std::abort();
  }
}
#endif // BlockTag_HPP
