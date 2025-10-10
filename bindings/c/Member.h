#ifndef Member_H
#define Member_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "Member.d.h"






void Member_name(const Member* self, DiplomatWrite* write);

bool Member_is_key(const Member* self);

void Member_destroy(Member* self);





#endif // Member_H
