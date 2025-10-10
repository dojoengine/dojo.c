#ifndef CallType_D_H
#define CallType_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum CallType {
  CallType_Execute = 0,
  CallType_ExecuteFromOutside = 1,
} CallType;

typedef struct CallType_option {union { CallType ok; }; bool is_ok; } CallType_option;



#endif // CallType_D_H
