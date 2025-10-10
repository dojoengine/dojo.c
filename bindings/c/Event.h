#ifndef Event_H
#define Event_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DojoError.d.h"

#include "Event.d.h"






void Event_keys(const Event* self, DiplomatWrite* write);

void Event_data(const Event* self, DiplomatWrite* write);

void Event_transaction_hash(const Event* self, DiplomatWrite* write);

typedef struct Event_from_json_result {union {Event* ok; DojoError* err;}; bool is_ok;} Event_from_json_result;
Event_from_json_result Event_from_json(DiplomatStringView json);

typedef struct Event_to_json_result {union { DojoError* err;}; bool is_ok;} Event_to_json_result;
Event_to_json_result Event_to_json(const Event* self, DiplomatWrite* write);

void Event_destroy(Event* self);





#endif // Event_H
