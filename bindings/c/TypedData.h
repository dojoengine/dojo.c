#ifndef TypedData_H
#define TypedData_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DojoError.d.h"

#include "TypedData.d.h"






typedef struct TypedData_new_from_json_result {union {TypedData* ok; DojoError* err;}; bool is_ok;} TypedData_new_from_json_result;
TypedData_new_from_json_result TypedData_new_from_json(DiplomatStringView json);

void TypedData_destroy(TypedData* self);





#endif // TypedData_H
