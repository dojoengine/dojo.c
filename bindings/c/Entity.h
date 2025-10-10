#ifndef Entity_H
#define Entity_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DojoError.d.h"

#include "Entity.d.h"






void Entity_hashed_keys(const Entity* self, DiplomatWrite* write);

uint32_t Entity_models_count(const Entity* self);

uint64_t Entity_created_at(const Entity* self);

uint64_t Entity_updated_at(const Entity* self);

uint64_t Entity_executed_at(const Entity* self);

typedef struct Entity_from_json_result {union {Entity* ok; DojoError* err;}; bool is_ok;} Entity_from_json_result;
Entity_from_json_result Entity_from_json(DiplomatStringView json);

typedef struct Entity_to_json_result {union { DojoError* err;}; bool is_ok;} Entity_to_json_result;
Entity_to_json_result Entity_to_json(const Entity* self, DiplomatWrite* write);

void Entity_destroy(Entity* self);





#endif // Entity_H
