#ifndef Provider_H
#define Provider_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DojoError.d.h"

#include "Provider.d.h"






typedef struct Provider_new_result {union {Provider* ok; DojoError* err;}; bool is_ok;} Provider_new_result;
Provider_new_result Provider_new(DiplomatStringView rpc_url);

typedef struct Provider_chain_id_result {union { DojoError* err;}; bool is_ok;} Provider_chain_id_result;
Provider_chain_id_result Provider_chain_id(const Provider* self, DiplomatWrite* write);

typedef struct Provider_block_number_result {union {uint64_t ok; DojoError* err;}; bool is_ok;} Provider_block_number_result;
Provider_block_number_result Provider_block_number(const Provider* self);

void Provider_destroy(Provider* self);





#endif // Provider_H
