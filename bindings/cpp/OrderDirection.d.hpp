#ifndef OrderDirection_D_HPP
#define OrderDirection_D_HPP

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
    enum OrderDirection {
      OrderDirection_Asc = 0,
      OrderDirection_Desc = 1,
    };

    typedef struct OrderDirection_option {union { OrderDirection ok; }; bool is_ok; } OrderDirection_option;
} // namespace capi
} // namespace

/**
 * Direction for ordering results
 */
class OrderDirection {
public:
  enum Value {
    Asc = 0,
    Desc = 1,
  };

  OrderDirection(): value(Value::Asc) {}

  // Implicit conversions between enum and ::Value
  constexpr OrderDirection(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline diplomat::capi::OrderDirection AsFFI() const;
  inline static OrderDirection FromFFI(diplomat::capi::OrderDirection c_enum);
private:
    Value value;
};


#endif // OrderDirection_D_HPP
