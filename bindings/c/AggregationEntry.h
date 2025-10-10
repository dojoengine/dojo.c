#ifndef AggregationEntry_H
#define AggregationEntry_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DojoError.d.h"

#include "AggregationEntry.d.h"






void AggregationEntry_id(const AggregationEntry* self, DiplomatWrite* write);

void AggregationEntry_aggregator_id(const AggregationEntry* self, DiplomatWrite* write);

void AggregationEntry_entity_id(const AggregationEntry* self, DiplomatWrite* write);

void AggregationEntry_model_id(const AggregationEntry* self, DiplomatWrite* write);

void AggregationEntry_value(const AggregationEntry* self, DiplomatWrite* write);

void AggregationEntry_display_value(const AggregationEntry* self, DiplomatWrite* write);

uint64_t AggregationEntry_position(const AggregationEntry* self);

uint64_t AggregationEntry_created_at(const AggregationEntry* self);

uint64_t AggregationEntry_updated_at(const AggregationEntry* self);

typedef struct AggregationEntry_from_json_result {union {AggregationEntry* ok; DojoError* err;}; bool is_ok;} AggregationEntry_from_json_result;
AggregationEntry_from_json_result AggregationEntry_from_json(DiplomatStringView json);

typedef struct AggregationEntry_to_json_result {union { DojoError* err;}; bool is_ok;} AggregationEntry_to_json_result;
AggregationEntry_to_json_result AggregationEntry_to_json(const AggregationEntry* self, DiplomatWrite* write);

void AggregationEntry_destroy(AggregationEntry* self);





#endif // AggregationEntry_H
