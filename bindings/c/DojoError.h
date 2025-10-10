#ifndef DojoError_H
#define DojoError_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ErrorType.d.h"

#include "DojoError.d.h"






DojoError* DojoError_new(ErrorType error_type, DiplomatStringView message);

void DojoError_message(const DojoError* self, DiplomatWrite* write);

ErrorType DojoError_error_type(const DojoError* self);

void DojoError_destroy(DojoError* self);





#endif // DojoError_H
