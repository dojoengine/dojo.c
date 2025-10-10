#ifndef PatternMatching_D_H
#define PatternMatching_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum PatternMatching {
  PatternMatching_FixedLen = 0,
  PatternMatching_VariableLen = 1,
} PatternMatching;

typedef struct PatternMatching_option {union { PatternMatching ok; }; bool is_ok; } PatternMatching_option;



#endif // PatternMatching_D_H
