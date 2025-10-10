#ifndef Token_H
#define Token_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DojoError.d.h"

#include "Token.d.h"






void Token_contract_address(const Token* self, DiplomatWrite* write);

void Token_name(const Token* self, DiplomatWrite* write);

void Token_symbol(const Token* self, DiplomatWrite* write);

uint8_t Token_decimals(const Token* self);

void Token_metadata(const Token* self, DiplomatWrite* write);

typedef struct Token_from_json_result {union {Token* ok; DojoError* err;}; bool is_ok;} Token_from_json_result;
Token_from_json_result Token_from_json(DiplomatStringView json);

typedef struct Token_to_json_result {union { DojoError* err;}; bool is_ok;} Token_to_json_result;
Token_to_json_result Token_to_json(const Token* self, DiplomatWrite* write);

void Token_destroy(Token* self);





#endif // Token_H
