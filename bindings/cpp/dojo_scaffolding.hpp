#pragma once

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#ifndef UNIFFI_CPP_INTERNALSTRUCTS
#define UNIFFI_CPP_INTERNALSTRUCTS
struct ForeignBytes {
    int32_t len;
    uint8_t *data;
};

struct RustBuffer {
    uint64_t capacity;
    uint64_t len;
    uint8_t *data;
};

struct RustCallStatus {
    int8_t code;
    RustBuffer error_buf;
};

#endif
struct UniffiVTableCallbackInterfaceEntityUpdateCallback {
    void * uniffi_free;
    void * uniffi_clone;
    void * on_update;
    void * on_error;
};
struct UniffiVTableCallbackInterfaceEventUpdateCallback {
    void * uniffi_free;
    void * uniffi_clone;
    void * on_update;
    void * on_error;
};
struct UniffiVTableCallbackInterfaceTokenBalanceUpdateCallback {
    void * uniffi_free;
    void * uniffi_clone;
    void * on_update;
    void * on_error;
};
struct UniffiVTableCallbackInterfaceTokenUpdateCallback {
    void * uniffi_free;
    void * uniffi_clone;
    void * on_update;
    void * on_error;
};
struct UniffiVTableCallbackInterfaceTransactionUpdateCallback {
    void * uniffi_free;
    void * uniffi_clone;
    void * on_update;
    void * on_error;
};
void * uniffi_dojo_uniffi_fn_clone_toriiclient(void * handle, RustCallStatus *out_status);
void uniffi_dojo_uniffi_fn_free_toriiclient(void * handle, RustCallStatus *out_status);
void * uniffi_dojo_uniffi_fn_constructor_toriiclient_new(RustBuffer torii_url, RustCallStatus *out_status);
void * uniffi_dojo_uniffi_fn_constructor_toriiclient_new_with_config(RustBuffer torii_url, uint64_t max_message_size, RustCallStatus *out_status);
RustBuffer uniffi_dojo_uniffi_fn_method_toriiclient_achievements(void * ptr, RustBuffer query, RustCallStatus *out_status);
RustBuffer uniffi_dojo_uniffi_fn_method_toriiclient_activities(void * ptr, RustBuffer query, RustCallStatus *out_status);
RustBuffer uniffi_dojo_uniffi_fn_method_toriiclient_aggregations(void * ptr, RustBuffer query, RustCallStatus *out_status);
void uniffi_dojo_uniffi_fn_method_toriiclient_cancel_subscription(void * ptr, uint64_t subscription_id, RustCallStatus *out_status);
RustBuffer uniffi_dojo_uniffi_fn_method_toriiclient_contracts(void * ptr, RustBuffer query, RustCallStatus *out_status);
RustBuffer uniffi_dojo_uniffi_fn_method_toriiclient_controllers(void * ptr, RustBuffer query, RustCallStatus *out_status);
RustBuffer uniffi_dojo_uniffi_fn_method_toriiclient_entities(void * ptr, RustBuffer query, RustCallStatus *out_status);
RustBuffer uniffi_dojo_uniffi_fn_method_toriiclient_event_messages(void * ptr, RustBuffer query, RustCallStatus *out_status);
RustBuffer uniffi_dojo_uniffi_fn_method_toriiclient_player_achievements(void * ptr, RustBuffer query, RustCallStatus *out_status);
RustBuffer uniffi_dojo_uniffi_fn_method_toriiclient_publish_message(void * ptr, RustBuffer message, RustCallStatus *out_status);
RustBuffer uniffi_dojo_uniffi_fn_method_toriiclient_publish_message_batch(void * ptr, RustBuffer messages, RustCallStatus *out_status);
RustBuffer uniffi_dojo_uniffi_fn_method_toriiclient_sql(void * ptr, RustBuffer query, RustCallStatus *out_status);
RustBuffer uniffi_dojo_uniffi_fn_method_toriiclient_starknet_events(void * ptr, RustBuffer query, RustCallStatus *out_status);
uint64_t uniffi_dojo_uniffi_fn_method_toriiclient_subscribe_entity_updates(void * ptr, RustBuffer clause, RustBuffer world_addresses, uint64_t callback, RustCallStatus *out_status);
uint64_t uniffi_dojo_uniffi_fn_method_toriiclient_subscribe_event_updates(void * ptr, RustBuffer keys, uint64_t callback, RustCallStatus *out_status);
uint64_t uniffi_dojo_uniffi_fn_method_toriiclient_subscribe_token_balance_updates(void * ptr, RustBuffer contract_addresses, RustBuffer account_addresses, RustBuffer token_ids, uint64_t callback, RustCallStatus *out_status);
uint64_t uniffi_dojo_uniffi_fn_method_toriiclient_subscribe_token_updates(void * ptr, RustBuffer contract_addresses, RustBuffer token_ids, uint64_t callback, RustCallStatus *out_status);
uint64_t uniffi_dojo_uniffi_fn_method_toriiclient_subscribe_transaction_updates(void * ptr, RustBuffer filter, uint64_t callback, RustCallStatus *out_status);
RustBuffer uniffi_dojo_uniffi_fn_method_toriiclient_token_balances(void * ptr, RustBuffer query, RustCallStatus *out_status);
RustBuffer uniffi_dojo_uniffi_fn_method_toriiclient_token_contracts(void * ptr, RustBuffer query, RustCallStatus *out_status);
RustBuffer uniffi_dojo_uniffi_fn_method_toriiclient_token_transfers(void * ptr, RustBuffer query, RustCallStatus *out_status);
RustBuffer uniffi_dojo_uniffi_fn_method_toriiclient_tokens(void * ptr, RustBuffer query, RustCallStatus *out_status);
RustBuffer uniffi_dojo_uniffi_fn_method_toriiclient_transactions(void * ptr, RustBuffer query, RustCallStatus *out_status);
RustBuffer uniffi_dojo_uniffi_fn_method_toriiclient_worlds(void * ptr, RustBuffer world_addresses, RustCallStatus *out_status);
void uniffi_dojo_uniffi_fn_init_callback_vtable_entityupdatecallback(UniffiVTableCallbackInterfaceEntityUpdateCallback & vtable);
void uniffi_dojo_uniffi_fn_init_callback_vtable_eventupdatecallback(UniffiVTableCallbackInterfaceEventUpdateCallback & vtable);
void uniffi_dojo_uniffi_fn_init_callback_vtable_tokenbalanceupdatecallback(UniffiVTableCallbackInterfaceTokenBalanceUpdateCallback & vtable);
void uniffi_dojo_uniffi_fn_init_callback_vtable_tokenupdatecallback(UniffiVTableCallbackInterfaceTokenUpdateCallback & vtable);
void uniffi_dojo_uniffi_fn_init_callback_vtable_transactionupdatecallback(UniffiVTableCallbackInterfaceTransactionUpdateCallback & vtable);
RustBuffer ffi_dojo_uniffi_rustbuffer_alloc(uint64_t size, RustCallStatus *out_status);
RustBuffer ffi_dojo_uniffi_rustbuffer_from_bytes(ForeignBytes bytes, RustCallStatus *out_status);
void ffi_dojo_uniffi_rustbuffer_free(RustBuffer buf, RustCallStatus *out_status);
RustBuffer ffi_dojo_uniffi_rustbuffer_reserve(RustBuffer buf, uint64_t additional, RustCallStatus *out_status);
uint16_t uniffi_dojo_uniffi_checksum_method_toriiclient_achievements();
uint16_t uniffi_dojo_uniffi_checksum_method_toriiclient_activities();
uint16_t uniffi_dojo_uniffi_checksum_method_toriiclient_aggregations();
uint16_t uniffi_dojo_uniffi_checksum_method_toriiclient_cancel_subscription();
uint16_t uniffi_dojo_uniffi_checksum_method_toriiclient_contracts();
uint16_t uniffi_dojo_uniffi_checksum_method_toriiclient_controllers();
uint16_t uniffi_dojo_uniffi_checksum_method_toriiclient_entities();
uint16_t uniffi_dojo_uniffi_checksum_method_toriiclient_event_messages();
uint16_t uniffi_dojo_uniffi_checksum_method_toriiclient_player_achievements();
uint16_t uniffi_dojo_uniffi_checksum_method_toriiclient_publish_message();
uint16_t uniffi_dojo_uniffi_checksum_method_toriiclient_publish_message_batch();
uint16_t uniffi_dojo_uniffi_checksum_method_toriiclient_sql();
uint16_t uniffi_dojo_uniffi_checksum_method_toriiclient_starknet_events();
uint16_t uniffi_dojo_uniffi_checksum_method_toriiclient_subscribe_entity_updates();
uint16_t uniffi_dojo_uniffi_checksum_method_toriiclient_subscribe_event_updates();
uint16_t uniffi_dojo_uniffi_checksum_method_toriiclient_subscribe_token_balance_updates();
uint16_t uniffi_dojo_uniffi_checksum_method_toriiclient_subscribe_token_updates();
uint16_t uniffi_dojo_uniffi_checksum_method_toriiclient_subscribe_transaction_updates();
uint16_t uniffi_dojo_uniffi_checksum_method_toriiclient_token_balances();
uint16_t uniffi_dojo_uniffi_checksum_method_toriiclient_token_contracts();
uint16_t uniffi_dojo_uniffi_checksum_method_toriiclient_token_transfers();
uint16_t uniffi_dojo_uniffi_checksum_method_toriiclient_tokens();
uint16_t uniffi_dojo_uniffi_checksum_method_toriiclient_transactions();
uint16_t uniffi_dojo_uniffi_checksum_method_toriiclient_worlds();
uint16_t uniffi_dojo_uniffi_checksum_constructor_toriiclient_new();
uint16_t uniffi_dojo_uniffi_checksum_constructor_toriiclient_new_with_config();
uint16_t uniffi_dojo_uniffi_checksum_method_entityupdatecallback_on_update();
uint16_t uniffi_dojo_uniffi_checksum_method_entityupdatecallback_on_error();
uint16_t uniffi_dojo_uniffi_checksum_method_eventupdatecallback_on_update();
uint16_t uniffi_dojo_uniffi_checksum_method_eventupdatecallback_on_error();
uint16_t uniffi_dojo_uniffi_checksum_method_tokenbalanceupdatecallback_on_update();
uint16_t uniffi_dojo_uniffi_checksum_method_tokenbalanceupdatecallback_on_error();
uint16_t uniffi_dojo_uniffi_checksum_method_tokenupdatecallback_on_update();
uint16_t uniffi_dojo_uniffi_checksum_method_tokenupdatecallback_on_error();
uint16_t uniffi_dojo_uniffi_checksum_method_transactionupdatecallback_on_update();
uint16_t uniffi_dojo_uniffi_checksum_method_transactionupdatecallback_on_error();
uint32_t ffi_dojo_uniffi_uniffi_contract_version();
#ifdef __cplusplus
}
#endif