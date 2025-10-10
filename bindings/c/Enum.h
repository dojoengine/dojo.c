#ifndef Enum_H
#define Enum_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DojoError.d.h"

#include "Enum.d.h"






void Enum_name(const Enum* self, DiplomatWrite* write);

uint8_t Enum_option(const Enum* self);

uint32_t Enum_options_count(const Enum* self);

typedef struct Enum_from_json_result {union {Enum* ok; DojoError* err;}; bool is_ok;} Enum_from_json_result;
Enum_from_json_result Enum_from_json(DiplomatStringView json);

typedef struct Enum_to_json_result {union { DojoError* err;}; bool is_ok;} Enum_to_json_result;
Enum_to_json_result Enum_to_json(const Enum* self, DiplomatWrite* write);

void Enum_destroy(Enum* self);





#endif // Enum_H
