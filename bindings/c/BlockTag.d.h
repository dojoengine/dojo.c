#ifndef BlockTag_D_H
#define BlockTag_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum BlockTag {
  BlockTag_Latest = 0,
  BlockTag_PreConfirmed = 1,
} BlockTag;

typedef struct BlockTag_option {union { BlockTag ok; }; bool is_ok; } BlockTag_option;



#endif // BlockTag_D_H
