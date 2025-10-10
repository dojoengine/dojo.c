#ifndef Activity_H
#define Activity_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DojoError.d.h"

#include "Activity.d.h"






void Activity_id(const Activity* self, DiplomatWrite* write);

void Activity_world_address(const Activity* self, DiplomatWrite* write);

void Activity_namespace(const Activity* self, DiplomatWrite* write);

void Activity_caller_address(const Activity* self, DiplomatWrite* write);

uint64_t Activity_session_start(const Activity* self);

uint64_t Activity_session_end(const Activity* self);

uint32_t Activity_action_count(const Activity* self);

uint64_t Activity_updated_at(const Activity* self);

typedef struct Activity_from_json_result {union {Activity* ok; DojoError* err;}; bool is_ok;} Activity_from_json_result;
Activity_from_json_result Activity_from_json(DiplomatStringView json);

typedef struct Activity_to_json_result {union { DojoError* err;}; bool is_ok;} Activity_to_json_result;
Activity_to_json_result Activity_to_json(const Activity* self, DiplomatWrite* write);

void Activity_destroy(Activity* self);





#endif // Activity_H
