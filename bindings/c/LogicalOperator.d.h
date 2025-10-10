#ifndef LogicalOperator_D_H
#define LogicalOperator_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum LogicalOperator {
  LogicalOperator_And = 0,
  LogicalOperator_Or = 1,
} LogicalOperator;

typedef struct LogicalOperator_option {union { LogicalOperator ok; }; bool is_ok; } LogicalOperator_option;



#endif // LogicalOperator_D_H
