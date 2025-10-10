#ifndef Primitive_H
#define Primitive_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DojoError.d.h"
#include "PrimitiveType.d.h"

#include "Primitive.d.h"






PrimitiveType Primitive_primitive_type(const Primitive* self);

typedef struct Primitive_from_json_result {union {Primitive* ok; DojoError* err;}; bool is_ok;} Primitive_from_json_result;
Primitive_from_json_result Primitive_from_json(DiplomatStringView json);

typedef struct Primitive_to_json_result {union { DojoError* err;}; bool is_ok;} Primitive_to_json_result;
Primitive_to_json_result Primitive_to_json(const Primitive* self, DiplomatWrite* write);

void Primitive_destroy(Primitive* self);





#endif // Primitive_H
