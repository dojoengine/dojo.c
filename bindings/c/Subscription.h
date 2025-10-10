#ifndef Subscription_H
#define Subscription_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "Subscription.d.h"






uint64_t Subscription_id(const Subscription* self);

void Subscription_destroy(Subscription* self);





#endif // Subscription_H
