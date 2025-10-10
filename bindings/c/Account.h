#ifndef Account_H
#define Account_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "CallList.d.h"
#include "DojoError.d.h"
#include "FieldElement.d.h"
#include "Provider.d.h"
#include "SigningKey.d.h"

#include "Account.d.h"






Account* Account_new(const Provider* provider, const SigningKey* signer, const FieldElement* address, const FieldElement* chain_id);

void Account_address(const Account* self, DiplomatWrite* write);

void Account_chain_id(const Account* self, DiplomatWrite* write);

typedef struct Account_execute_result {union { DojoError* err;}; bool is_ok;} Account_execute_result;
Account_execute_result Account_execute(const Account* self, const CallList* calls, DiplomatWrite* write);

typedef struct Account_nonce_result {union {uint64_t ok; DojoError* err;}; bool is_ok;} Account_nonce_result;
Account_nonce_result Account_nonce(const Account* self);

void Account_destroy(Account* self);





#endif // Account_H
