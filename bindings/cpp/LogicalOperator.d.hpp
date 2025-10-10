#ifndef LogicalOperator_D_HPP
#define LogicalOperator_D_HPP

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
    enum LogicalOperator {
      LogicalOperator_And = 0,
      LogicalOperator_Or = 1,
    };

    typedef struct LogicalOperator_option {union { LogicalOperator ok; }; bool is_ok; } LogicalOperator_option;
} // namespace capi
} // namespace

/**
 * Logical operator for combining clauses
 */
class LogicalOperator {
public:
  enum Value {
    And = 0,
    Or = 1,
  };

  LogicalOperator(): value(Value::And) {}

  // Implicit conversions between enum and ::Value
  constexpr LogicalOperator(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline diplomat::capi::LogicalOperator AsFFI() const;
  inline static LogicalOperator FromFFI(diplomat::capi::LogicalOperator c_enum);
private:
    Value value;
};


#endif // LogicalOperator_D_HPP
