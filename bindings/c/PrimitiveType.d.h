#ifndef PrimitiveType_D_H
#define PrimitiveType_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum PrimitiveType {
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
} PrimitiveType;

typedef struct PrimitiveType_option {union { PrimitiveType ok; }; bool is_ok; } PrimitiveType_option;



#endif // PrimitiveType_D_H
