#ifndef BlockId_H
#define BlockId_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BlockTag.d.h"
#include "FieldElement.d.h"

#include "BlockId.d.h"






BlockId* BlockId_from_hash(const FieldElement* hash);

BlockId* BlockId_from_number(uint64_t number);

BlockId* BlockId_from_tag(BlockTag tag);

void BlockId_destroy(BlockId* self);





#endif // BlockId_H
