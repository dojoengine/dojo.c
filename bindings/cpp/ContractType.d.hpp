#ifndef ContractType_D_HPP
#define ContractType_D_HPP

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
    enum ContractType {
      ContractType_WORLD = 0,
      ContractType_ERC20 = 1,
      ContractType_ERC721 = 2,
      ContractType_ERC1155 = 3,
      ContractType_UDC = 4,
      ContractType_OTHER = 5,
    };

    typedef struct ContractType_option {union { ContractType ok; }; bool is_ok; } ContractType_option;
} // namespace capi
} // namespace

/**
 * Type of contract
 */
class ContractType {
public:
  enum Value {
    WORLD = 0,
    ERC20 = 1,
    ERC721 = 2,
    ERC1155 = 3,
    UDC = 4,
    OTHER = 5,
  };

  ContractType(): value(Value::WORLD) {}

  // Implicit conversions between enum and ::Value
  constexpr ContractType(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline diplomat::capi::ContractType AsFFI() const;
  inline static ContractType FromFFI(diplomat::capi::ContractType c_enum);
private:
    Value value;
};


#endif // ContractType_D_HPP
