#ifndef BlockTag_D_HPP
#define BlockTag_D_HPP

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
    enum BlockTag {
      BlockTag_Latest = 0,
      BlockTag_PreConfirmed = 1,
    };

    typedef struct BlockTag_option {union { BlockTag ok; }; bool is_ok; } BlockTag_option;
} // namespace capi
} // namespace

/**
 * Block tag for identifying specific blocks
 */
class BlockTag {
public:
  enum Value {
    Latest = 0,
    PreConfirmed = 1,
  };

  BlockTag(): value(Value::Latest) {}

  // Implicit conversions between enum and ::Value
  constexpr BlockTag(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline diplomat::capi::BlockTag AsFFI() const;
  inline static BlockTag FromFFI(diplomat::capi::BlockTag c_enum);
private:
    Value value;
};


#endif // BlockTag_D_HPP
