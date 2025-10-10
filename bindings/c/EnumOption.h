#ifndef EnumOption_H
#define EnumOption_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "EnumOption.d.h"






void EnumOption_name(const EnumOption* self, DiplomatWrite* write);

void EnumOption_destroy(EnumOption* self);





#endif // EnumOption_H
