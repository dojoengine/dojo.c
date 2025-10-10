#ifndef Call_H
#define Call_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DojoError.d.h"
#include "FieldElement.d.h"

#include "Call.d.h"






Call* Call_new(const FieldElement* to, const FieldElement* selector);

void Call_push_calldata(Call* self, const FieldElement* felt);

typedef struct Call_new_from_selector_name_result {union {Call* ok; DojoError* err;}; bool is_ok;} Call_new_from_selector_name_result;
Call_new_from_selector_name_result Call_new_from_selector_name(const FieldElement* to, DiplomatStringView selector_name);

void Call_destroy(Call* self);





#endif // Call_H
