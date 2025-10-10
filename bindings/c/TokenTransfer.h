#ifndef TokenTransfer_H
#define TokenTransfer_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DojoError.d.h"

#include "TokenTransfer.d.h"






void TokenTransfer_from_address(const TokenTransfer* self, DiplomatWrite* write);

void TokenTransfer_to_address(const TokenTransfer* self, DiplomatWrite* write);

void TokenTransfer_contract_address(const TokenTransfer* self, DiplomatWrite* write);

void TokenTransfer_amount(const TokenTransfer* self, DiplomatWrite* write);

uint64_t TokenTransfer_executed_at(const TokenTransfer* self);

typedef struct TokenTransfer_from_json_result {union {TokenTransfer* ok; DojoError* err;}; bool is_ok;} TokenTransfer_from_json_result;
TokenTransfer_from_json_result TokenTransfer_from_json(DiplomatStringView json);

typedef struct TokenTransfer_to_json_result {union { DojoError* err;}; bool is_ok;} TokenTransfer_to_json_result;
TokenTransfer_to_json_result TokenTransfer_to_json(const TokenTransfer* self, DiplomatWrite* write);

void TokenTransfer_destroy(TokenTransfer* self);





#endif // TokenTransfer_H
