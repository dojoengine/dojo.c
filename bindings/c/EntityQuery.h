#ifndef EntityQuery_H
#define EntityQuery_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "EntityQuery.d.h"






EntityQuery* EntityQuery_new(void);

void EntityQuery_set_limit(EntityQuery* self, uint32_t limit);

void EntityQuery_set_offset(EntityQuery* self, uint32_t offset);

void EntityQuery_destroy(EntityQuery* self);





#endif // EntityQuery_H
