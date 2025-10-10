#ifndef CallType_D_HPP
#define CallType_D_HPP

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
    enum CallType {
      CallType_Execute = 0,
      CallType_ExecuteFromOutside = 1,
    };

    typedef struct CallType_option {union { CallType ok; }; bool is_ok; } CallType_option;
} // namespace capi
} // namespace

/**
 * Type of call in a transaction
 */
class CallType {
public:
  enum Value {
    Execute = 0,
    ExecuteFromOutside = 1,
  };

  CallType(): value(Value::Execute) {}

  // Implicit conversions between enum and ::Value
  constexpr CallType(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline diplomat::capi::CallType AsFFI() const;
  inline static CallType FromFFI(diplomat::capi::CallType c_enum);
private:
    Value value;
};


#endif // CallType_D_HPP
