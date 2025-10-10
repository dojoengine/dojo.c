#ifndef U256_H
#define U256_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DojoError.d.h"

#include "U256.d.h"






typedef struct U256_new_from_hex_result {union {U256* ok; DojoError* err;}; bool is_ok;} U256_new_from_hex_result;
U256_new_from_hex_result U256_new_from_hex(DiplomatStringView hex);

typedef struct U256_new_from_bytes_result {union {U256* ok; DojoError* err;}; bool is_ok;} U256_new_from_bytes_result;
U256_new_from_bytes_result U256_new_from_bytes(DiplomatU8View bytes);

void U256_to_hex(const U256* self, DiplomatWrite* write);

void U256_to_bytes(const U256* self, DiplomatU8ViewMut result);

void U256_destroy(U256* self);





#endif // U256_H
