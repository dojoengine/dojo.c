#ifndef Model_H
#define Model_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DojoError.d.h"

#include "Model.d.h"






void Model_name(const Model* self, DiplomatWrite* write);

void Model_namespace(const Model* self, DiplomatWrite* write);

void Model_selector(const Model* self, DiplomatWrite* write);

void Model_class_hash(const Model* self, DiplomatWrite* write);

void Model_contract_address(const Model* self, DiplomatWrite* write);

uint32_t Model_packed_size(const Model* self);

uint32_t Model_unpacked_size(const Model* self);

bool Model_use_legacy_store(const Model* self);

typedef struct Model_from_json_result {union {Model* ok; DojoError* err;}; bool is_ok;} Model_from_json_result;
Model_from_json_result Model_from_json(DiplomatStringView json);

typedef struct Model_to_json_result {union { DojoError* err;}; bool is_ok;} Model_to_json_result;
Model_to_json_result Model_to_json(const Model* self, DiplomatWrite* write);

void Model_destroy(Model* self);





#endif // Model_H
