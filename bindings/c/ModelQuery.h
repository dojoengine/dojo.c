#ifndef ModelQuery_H
#define ModelQuery_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DojoError.d.h"

#include "ModelQuery.d.h"






ModelQuery* ModelQuery_new(void);

typedef struct ModelQuery_add_model_result {union { DojoError* err;}; bool is_ok;} ModelQuery_add_model_result;
ModelQuery_add_model_result ModelQuery_add_model(ModelQuery* self, DiplomatStringView model_name);

void ModelQuery_destroy(ModelQuery* self);





#endif // ModelQuery_H
