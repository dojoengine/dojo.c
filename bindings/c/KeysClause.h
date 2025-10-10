#ifndef KeysClause_H
#define KeysClause_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DojoError.d.h"
#include "FieldElement.d.h"

#include "KeysClause.d.h"






KeysClause* KeysClause_new(void);

void KeysClause_add_key(KeysClause* self, const FieldElement* key);

typedef struct KeysClause_add_model_result {union { DojoError* err;}; bool is_ok;} KeysClause_add_model_result;
KeysClause_add_model_result KeysClause_add_model(KeysClause* self, DiplomatStringView model);

void KeysClause_destroy(KeysClause* self);





#endif // KeysClause_H
