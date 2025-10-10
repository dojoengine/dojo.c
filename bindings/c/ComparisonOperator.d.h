#ifndef ComparisonOperator_D_H
#define ComparisonOperator_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum ComparisonOperator {
  ComparisonOperator_Eq = 0,
  ComparisonOperator_Neq = 1,
  ComparisonOperator_Gt = 2,
  ComparisonOperator_Gte = 3,
  ComparisonOperator_Lt = 4,
  ComparisonOperator_Lte = 5,
  ComparisonOperator_In = 6,
  ComparisonOperator_NotIn = 7,
  ComparisonOperator_Contains = 8,
  ComparisonOperator_ContainsAll = 9,
  ComparisonOperator_ContainsAny = 10,
  ComparisonOperator_ArrayLengthEq = 11,
  ComparisonOperator_ArrayLengthGt = 12,
  ComparisonOperator_ArrayLengthLt = 13,
} ComparisonOperator;

typedef struct ComparisonOperator_option {union { ComparisonOperator ok; }; bool is_ok; } ComparisonOperator_option;



#endif // ComparisonOperator_D_H
