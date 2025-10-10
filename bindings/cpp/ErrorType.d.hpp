#ifndef ErrorType_D_HPP
#define ErrorType_D_HPP

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
    enum ErrorType {
      ErrorType_ClientError = 0,
      ErrorType_ParseError = 1,
      ErrorType_EntityError = 2,
      ErrorType_QueryError = 3,
      ErrorType_SubscriptionError = 4,
      ErrorType_TransactionError = 5,
      ErrorType_AccountError = 6,
      ErrorType_SigningError = 7,
      ErrorType_ProviderError = 8,
      ErrorType_StorageError = 9,
      ErrorType_ControllerError = 10,
      ErrorType_InvalidInput = 11,
      ErrorType_RuntimeError = 12,
    };

    typedef struct ErrorType_option {union { ErrorType ok; }; bool is_ok; } ErrorType_option;
} // namespace capi
} // namespace

class ErrorType {
public:
  enum Value {
    ClientError = 0,
    ParseError = 1,
    EntityError = 2,
    QueryError = 3,
    SubscriptionError = 4,
    TransactionError = 5,
    AccountError = 6,
    SigningError = 7,
    ProviderError = 8,
    StorageError = 9,
    ControllerError = 10,
    InvalidInput = 11,
    RuntimeError = 12,
  };

  ErrorType(): value(Value::ClientError) {}

  // Implicit conversions between enum and ::Value
  constexpr ErrorType(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline diplomat::capi::ErrorType AsFFI() const;
  inline static ErrorType FromFFI(diplomat::capi::ErrorType c_enum);
private:
    Value value;
};


#endif // ErrorType_D_HPP
