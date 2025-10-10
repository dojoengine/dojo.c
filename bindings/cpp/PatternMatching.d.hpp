#ifndef PatternMatching_D_HPP
#define PatternMatching_D_HPP

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
    enum PatternMatching {
      PatternMatching_FixedLen = 0,
      PatternMatching_VariableLen = 1,
    };

    typedef struct PatternMatching_option {union { PatternMatching ok; }; bool is_ok; } PatternMatching_option;
} // namespace capi
} // namespace

/**
 * Pattern matching mode for key queries
 */
class PatternMatching {
public:
  enum Value {
    FixedLen = 0,
    VariableLen = 1,
  };

  PatternMatching(): value(Value::FixedLen) {}

  // Implicit conversions between enum and ::Value
  constexpr PatternMatching(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline diplomat::capi::PatternMatching AsFFI() const;
  inline static PatternMatching FromFFI(diplomat::capi::PatternMatching c_enum);
private:
    Value value;
};


#endif // PatternMatching_D_HPP
