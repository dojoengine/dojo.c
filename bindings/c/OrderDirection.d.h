#ifndef OrderDirection_D_H
#define OrderDirection_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum OrderDirection {
  OrderDirection_Asc = 0,
  OrderDirection_Desc = 1,
} OrderDirection;

typedef struct OrderDirection_option {union { OrderDirection ok; }; bool is_ok; } OrderDirection_option;



#endif // OrderDirection_D_H
