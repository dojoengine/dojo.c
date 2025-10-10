#ifndef PrimitiveType_D_HPP
#define PrimitiveType_D_HPP

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
    enum PrimitiveType {
      PrimitiveType_I8 = 0,
      PrimitiveType_I16 = 1,
      PrimitiveType_I32 = 2,
      PrimitiveType_I64 = 3,
      PrimitiveType_I128 = 4,
      PrimitiveType_U8 = 5,
      PrimitiveType_U16 = 6,
      PrimitiveType_U32 = 7,
      PrimitiveType_U64 = 8,
      PrimitiveType_U128 = 9,
      PrimitiveType_U256 = 10,
      PrimitiveType_Bool = 11,
      PrimitiveType_Felt252 = 12,
      PrimitiveType_ClassHash = 13,
      PrimitiveType_ContractAddress = 14,
      PrimitiveType_EthAddress = 15,
    };

    typedef struct PrimitiveType_option {union { PrimitiveType ok; }; bool is_ok; } PrimitiveType_option;
} // namespace capi
} // namespace

/**
 * Type of primitive
 */
class PrimitiveType {
public:
  enum Value {
    I8 = 0,
    I16 = 1,
    I32 = 2,
    I64 = 3,
    I128 = 4,
    U8 = 5,
    U16 = 6,
    U32 = 7,
    U64 = 8,
    U128 = 9,
    U256 = 10,
    Bool = 11,
    Felt252 = 12,
    ClassHash = 13,
    ContractAddress = 14,
    EthAddress = 15,
  };

  PrimitiveType(): value(Value::I8) {}

  // Implicit conversions between enum and ::Value
  constexpr PrimitiveType(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline diplomat::capi::PrimitiveType AsFFI() const;
  inline static PrimitiveType FromFFI(diplomat::capi::PrimitiveType c_enum);
private:
    Value value;
};


#endif // PrimitiveType_D_HPP
