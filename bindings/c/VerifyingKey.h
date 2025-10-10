#ifndef VerifyingKey_H
#define VerifyingKey_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DojoError.d.h"
#include "FieldElement.d.h"
#include "Signature.d.h"

#include "VerifyingKey.d.h"






void VerifyingKey_scalar(const VerifyingKey* self, DiplomatWrite* write);

typedef struct VerifyingKey_verify_result {union {bool ok; DojoError* err;}; bool is_ok;} VerifyingKey_verify_result;
VerifyingKey_verify_result VerifyingKey_verify(const VerifyingKey* self, const FieldElement* hash, const Signature* signature);

void VerifyingKey_destroy(VerifyingKey* self);





#endif // VerifyingKey_H
