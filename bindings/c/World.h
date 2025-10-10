#ifndef World_H
#define World_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DojoError.d.h"

#include "World.d.h"






void World_world_address(const World* self, DiplomatWrite* write);

uint32_t World_models_count(const World* self);

typedef struct World_from_json_result {union {World* ok; DojoError* err;}; bool is_ok;} World_from_json_result;
World_from_json_result World_from_json(DiplomatStringView json);

typedef struct World_to_json_result {union { DojoError* err;}; bool is_ok;} World_to_json_result;
World_to_json_result World_to_json(const World* self, DiplomatWrite* write);

void World_destroy(World* self);





#endif // World_H
