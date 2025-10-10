#ifndef PaginationDirection_D_H
#define PaginationDirection_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum PaginationDirection {
  PaginationDirection_Forward = 0,
  PaginationDirection_Backward = 1,
} PaginationDirection;

typedef struct PaginationDirection_option {union { PaginationDirection ok; }; bool is_ok; } PaginationDirection_option;



#endif // PaginationDirection_D_H
