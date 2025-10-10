#ifndef Achievement_H
#define Achievement_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DojoError.d.h"

#include "Achievement.d.h"






void Achievement_id(const Achievement* self, DiplomatWrite* write);

void Achievement_world_address(const Achievement* self, DiplomatWrite* write);

void Achievement_namespace(const Achievement* self, DiplomatWrite* write);

void Achievement_title(const Achievement* self, DiplomatWrite* write);

void Achievement_description(const Achievement* self, DiplomatWrite* write);

bool Achievement_hidden(const Achievement* self);

void Achievement_icon(const Achievement* self, DiplomatWrite* write);

uint32_t Achievement_points(const Achievement* self);

typedef struct Achievement_from_json_result {union {Achievement* ok; DojoError* err;}; bool is_ok;} Achievement_from_json_result;
Achievement_from_json_result Achievement_from_json(DiplomatStringView json);

typedef struct Achievement_to_json_result {union { DojoError* err;}; bool is_ok;} Achievement_to_json_result;
Achievement_to_json_result Achievement_to_json(const Achievement* self, DiplomatWrite* write);

void Achievement_destroy(Achievement* self);





#endif // Achievement_H
