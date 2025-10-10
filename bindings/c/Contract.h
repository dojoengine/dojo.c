#ifndef Contract_H
#define Contract_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DojoError.d.h"

#include "Contract.d.h"






void Contract_contract_address(const Contract* self, DiplomatWrite* write);

void Contract_contract_type(const Contract* self, DiplomatWrite* write);

uint64_t Contract_head(const Contract* self);

uint64_t Contract_tps(const Contract* self);

uint64_t Contract_created_at(const Contract* self);

uint64_t Contract_updated_at(const Contract* self);

typedef struct Contract_from_json_result {union {Contract* ok; DojoError* err;}; bool is_ok;} Contract_from_json_result;
Contract_from_json_result Contract_from_json(DiplomatStringView json);

typedef struct Contract_to_json_result {union { DojoError* err;}; bool is_ok;} Contract_to_json_result;
Contract_to_json_result Contract_to_json(const Contract* self, DiplomatWrite* write);

void Contract_destroy(Contract* self);





#endif // Contract_H
