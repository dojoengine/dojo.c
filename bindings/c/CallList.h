#ifndef CallList_H
#define CallList_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "Call.d.h"

#include "CallList.d.h"






CallList* CallList_new(void);

void CallList_add_call(CallList* self, const Call* call);

size_t CallList_len(const CallList* self);

void CallList_destroy(CallList* self);





#endif // CallList_H
