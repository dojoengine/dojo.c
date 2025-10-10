#ifndef TokenBalance_H
#define TokenBalance_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DojoError.d.h"

#include "TokenBalance.d.h"






void TokenBalance_account_address(const TokenBalance* self, DiplomatWrite* write);

void TokenBalance_contract_address(const TokenBalance* self, DiplomatWrite* write);

void TokenBalance_balance(const TokenBalance* self, DiplomatWrite* write);

typedef struct TokenBalance_from_json_result {union {TokenBalance* ok; DojoError* err;}; bool is_ok;} TokenBalance_from_json_result;
TokenBalance_from_json_result TokenBalance_from_json(DiplomatStringView json);

typedef struct TokenBalance_to_json_result {union { DojoError* err;}; bool is_ok;} TokenBalance_to_json_result;
TokenBalance_to_json_result TokenBalance_to_json(const TokenBalance* self, DiplomatWrite* write);

void TokenBalance_destroy(TokenBalance* self);





#endif // TokenBalance_H
