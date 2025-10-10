#ifndef Transaction_H
#define Transaction_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DojoError.d.h"

#include "Transaction.d.h"






void Transaction_transaction_hash(const Transaction* self, DiplomatWrite* write);

void Transaction_sender_address(const Transaction* self, DiplomatWrite* write);

void Transaction_max_fee(const Transaction* self, DiplomatWrite* write);

void Transaction_signature(const Transaction* self, DiplomatWrite* write);

void Transaction_nonce(const Transaction* self, DiplomatWrite* write);

uint64_t Transaction_block_timestamp(const Transaction* self);

uint64_t Transaction_block_number(const Transaction* self);

typedef struct Transaction_from_json_result {union {Transaction* ok; DojoError* err;}; bool is_ok;} Transaction_from_json_result;
Transaction_from_json_result Transaction_from_json(DiplomatStringView json);

typedef struct Transaction_to_json_result {union { DojoError* err;}; bool is_ok;} Transaction_to_json_result;
Transaction_to_json_result Transaction_to_json(const Transaction* self, DiplomatWrite* write);

void Transaction_destroy(Transaction* self);





#endif // Transaction_H
