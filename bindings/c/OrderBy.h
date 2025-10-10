#ifndef OrderBy_H
#define OrderBy_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DojoError.d.h"
#include "OrderDirection.d.h"

#include "OrderBy.d.h"






typedef struct OrderBy_new_result {union {OrderBy* ok; DojoError* err;}; bool is_ok;} OrderBy_new_result;
OrderBy_new_result OrderBy_new(DiplomatStringView field, OrderDirection direction);

void OrderBy_destroy(OrderBy* self);





#endif // OrderBy_H
