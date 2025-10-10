#ifndef FieldElement_H
#define FieldElement_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DojoError.d.h"

#include "FieldElement.d.h"






typedef struct FieldElement_new_from_hex_result {union {FieldElement* ok; DojoError* err;}; bool is_ok;} FieldElement_new_from_hex_result;
FieldElement_new_from_hex_result FieldElement_new_from_hex(DiplomatStringView hex);

FieldElement* FieldElement_new_from_bytes(DiplomatU8View bytes);

void FieldElement_to_hex(const FieldElement* self, DiplomatWrite* write);

void FieldElement_to_bytes(const FieldElement* self, DiplomatU8ViewMut result);

void FieldElement_destroy(FieldElement* self);





#endif // FieldElement_H
