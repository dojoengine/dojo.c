#ifndef ContractType_D_H
#define ContractType_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum ContractType {
  ContractType_WORLD = 0,
  ContractType_ERC20 = 1,
  ContractType_ERC721 = 2,
  ContractType_ERC1155 = 3,
  ContractType_UDC = 4,
  ContractType_OTHER = 5,
} ContractType;

typedef struct ContractType_option {union { ContractType ok; }; bool is_ok; } ContractType_option;



#endif // ContractType_D_H
