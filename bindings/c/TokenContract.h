#ifndef TokenContract_H
#define TokenContract_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DojoError.d.h"

#include "TokenContract.d.h"






void TokenContract_contract_address(const TokenContract* self, DiplomatWrite* write);

void TokenContract_contract_type(const TokenContract* self, DiplomatWrite* write);

typedef struct TokenContract_from_json_result {union {TokenContract* ok; DojoError* err;}; bool is_ok;} TokenContract_from_json_result;
TokenContract_from_json_result TokenContract_from_json(DiplomatStringView json);

typedef struct TokenContract_to_json_result {union { DojoError* err;}; bool is_ok;} TokenContract_to_json_result;
TokenContract_to_json_result TokenContract_to_json(const TokenContract* self, DiplomatWrite* write);

void TokenContract_destroy(TokenContract* self);





#endif // TokenContract_H
