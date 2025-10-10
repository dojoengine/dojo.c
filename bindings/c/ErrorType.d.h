#ifndef ErrorType_D_H
#define ErrorType_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum ErrorType {
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
} ErrorType;

typedef struct ErrorType_option {union { ErrorType ok; }; bool is_ok; } ErrorType_option;



#endif // ErrorType_D_H
