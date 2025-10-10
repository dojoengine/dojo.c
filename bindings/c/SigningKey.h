#ifndef SigningKey_H
#define SigningKey_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DojoError.d.h"
#include "FieldElement.d.h"
#include "Signature.d.h"
#include "VerifyingKey.d.h"

#include "SigningKey.d.h"






typedef struct SigningKey_new_result {union {SigningKey* ok; DojoError* err;}; bool is_ok;} SigningKey_new_result;
SigningKey_new_result SigningKey_new(DiplomatStringView secret_scalar);

SigningKey* SigningKey_from_random(void);

void SigningKey_secret_scalar(const SigningKey* self, DiplomatWrite* write);

typedef struct SigningKey_sign_result {union {Signature* ok; DojoError* err;}; bool is_ok;} SigningKey_sign_result;
SigningKey_sign_result SigningKey_sign(const SigningKey* self, const FieldElement* hash);

VerifyingKey* SigningKey_verifying_key(const SigningKey* self);

void SigningKey_destroy(SigningKey* self);





#endif // SigningKey_H
