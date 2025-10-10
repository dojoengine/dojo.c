#ifndef ComparisonOperator_D_HPP
#define ComparisonOperator_D_HPP

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
    enum ComparisonOperator {
      ComparisonOperator_Eq = 0,
      ComparisonOperator_Neq = 1,
      ComparisonOperator_Gt = 2,
      ComparisonOperator_Gte = 3,
      ComparisonOperator_Lt = 4,
      ComparisonOperator_Lte = 5,
      ComparisonOperator_In = 6,
      ComparisonOperator_NotIn = 7,
      ComparisonOperator_Contains = 8,
      ComparisonOperator_ContainsAll = 9,
      ComparisonOperator_ContainsAny = 10,
      ComparisonOperator_ArrayLengthEq = 11,
      ComparisonOperator_ArrayLengthGt = 12,
      ComparisonOperator_ArrayLengthLt = 13,
    };

    typedef struct ComparisonOperator_option {union { ComparisonOperator ok; }; bool is_ok; } ComparisonOperator_option;
} // namespace capi
} // namespace

/**
 * Comparison operators for member clauses
 */
class ComparisonOperator {
public:
  enum Value {
    Eq = 0,
    Neq = 1,
    Gt = 2,
    Gte = 3,
    Lt = 4,
    Lte = 5,
    In = 6,
    NotIn = 7,
    Contains = 8,
    ContainsAll = 9,
    ContainsAny = 10,
    ArrayLengthEq = 11,
    ArrayLengthGt = 12,
    ArrayLengthLt = 13,
  };

  ComparisonOperator(): value(Value::Eq) {}

  // Implicit conversions between enum and ::Value
  constexpr ComparisonOperator(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline diplomat::capi::ComparisonOperator AsFFI() const;
  inline static ComparisonOperator FromFFI(diplomat::capi::ComparisonOperator c_enum);
private:
    Value value;
};


#endif // ComparisonOperator_D_HPP
