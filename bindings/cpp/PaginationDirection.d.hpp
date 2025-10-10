#ifndef PaginationDirection_D_HPP
#define PaginationDirection_D_HPP

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
    enum PaginationDirection {
      PaginationDirection_Forward = 0,
      PaginationDirection_Backward = 1,
    };

    typedef struct PaginationDirection_option {union { PaginationDirection ok; }; bool is_ok; } PaginationDirection_option;
} // namespace capi
} // namespace

/**
 * Direction for pagination
 */
class PaginationDirection {
public:
  enum Value {
    Forward = 0,
    Backward = 1,
  };

  PaginationDirection(): value(Value::Forward) {}

  // Implicit conversions between enum and ::Value
  constexpr PaginationDirection(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline diplomat::capi::PaginationDirection AsFFI() const;
  inline static PaginationDirection FromFFI(diplomat::capi::PaginationDirection c_enum);
private:
    Value value;
};


#endif // PaginationDirection_D_HPP
