#ifndef Struct_H
#define Struct_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DojoError.d.h"

#include "Struct.d.h"






void Struct_name(const Struct* self, DiplomatWrite* write);

uint32_t Struct_children_count(const Struct* self);

typedef struct Struct_from_json_result {union {Struct* ok; DojoError* err;}; bool is_ok;} Struct_from_json_result;
Struct_from_json_result Struct_from_json(DiplomatStringView json);

typedef struct Struct_to_json_result {union { DojoError* err;}; bool is_ok;} Struct_to_json_result;
Struct_to_json_result Struct_to_json(const Struct* self, DiplomatWrite* write);

void Struct_destroy(Struct* self);





#endif // Struct_H
