#ifndef ToriiClient_H
#define ToriiClient_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DojoError.d.h"
#include "FieldElement.d.h"

#include "ToriiClient.d.h"






typedef struct ToriiClient_new_result {union {ToriiClient* ok; DojoError* err;}; bool is_ok;} ToriiClient_new_result;
ToriiClient_new_result ToriiClient_new(DiplomatStringView torii_url);

typedef struct ToriiClient_info_result {union { DojoError* err;}; bool is_ok;} ToriiClient_info_result;
ToriiClient_info_result ToriiClient_info(const ToriiClient* self, DiplomatWrite* write);

typedef struct ToriiClient_publish_message_result {union { DojoError* err;}; bool is_ok;} ToriiClient_publish_message_result;
ToriiClient_publish_message_result ToriiClient_publish_message(const ToriiClient* self, DiplomatStringView message_json, const FieldElement* signature_r, const FieldElement* signature_s, const FieldElement* world_address, DiplomatWrite* write);

void ToriiClient_destroy(ToriiClient* self);





#endif // ToriiClient_H
