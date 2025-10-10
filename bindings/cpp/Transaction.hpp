#ifndef Transaction_HPP
#define Transaction_HPP

#include "Transaction.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "DojoError.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {

    void Transaction_transaction_hash(const diplomat::capi::Transaction* self, diplomat::capi::DiplomatWrite* write);

    void Transaction_sender_address(const diplomat::capi::Transaction* self, diplomat::capi::DiplomatWrite* write);

    void Transaction_max_fee(const diplomat::capi::Transaction* self, diplomat::capi::DiplomatWrite* write);

    void Transaction_signature(const diplomat::capi::Transaction* self, diplomat::capi::DiplomatWrite* write);

    void Transaction_nonce(const diplomat::capi::Transaction* self, diplomat::capi::DiplomatWrite* write);

    uint64_t Transaction_block_timestamp(const diplomat::capi::Transaction* self);

    uint64_t Transaction_block_number(const diplomat::capi::Transaction* self);

    typedef struct Transaction_from_json_result {union {diplomat::capi::Transaction* ok; diplomat::capi::DojoError* err;}; bool is_ok;} Transaction_from_json_result;
    Transaction_from_json_result Transaction_from_json(diplomat::capi::DiplomatStringView json);

    typedef struct Transaction_to_json_result {union { diplomat::capi::DojoError* err;}; bool is_ok;} Transaction_to_json_result;
    Transaction_to_json_result Transaction_to_json(const diplomat::capi::Transaction* self, diplomat::capi::DiplomatWrite* write);

    void Transaction_destroy(Transaction* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::string Transaction::transaction_hash() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::Transaction_transaction_hash(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void Transaction::transaction_hash_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::Transaction_transaction_hash(this->AsFFI(),
    &write);
}

inline std::string Transaction::sender_address() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::Transaction_sender_address(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void Transaction::sender_address_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::Transaction_sender_address(this->AsFFI(),
    &write);
}

inline std::string Transaction::max_fee() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::Transaction_max_fee(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void Transaction::max_fee_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::Transaction_max_fee(this->AsFFI(),
    &write);
}

inline std::string Transaction::signature() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::Transaction_signature(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void Transaction::signature_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::Transaction_signature(this->AsFFI(),
    &write);
}

inline std::string Transaction::nonce() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::Transaction_nonce(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void Transaction::nonce_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::Transaction_nonce(this->AsFFI(),
    &write);
}

inline uint64_t Transaction::block_timestamp() const {
  auto result = diplomat::capi::Transaction_block_timestamp(this->AsFFI());
  return result;
}

inline uint64_t Transaction::block_number() const {
  auto result = diplomat::capi::Transaction_block_number(this->AsFFI());
  return result;
}

inline diplomat::result<std::unique_ptr<Transaction>, std::unique_ptr<DojoError>> Transaction::from_json(std::string_view json) {
  auto result = diplomat::capi::Transaction_from_json({json.data(), json.size()});
  return result.is_ok ? diplomat::result<std::unique_ptr<Transaction>, std::unique_ptr<DojoError>>(diplomat::Ok<std::unique_ptr<Transaction>>(std::unique_ptr<Transaction>(Transaction::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<Transaction>, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<DojoError>> Transaction::to_json() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::Transaction_to_json(this->AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, std::unique_ptr<DojoError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> Transaction::to_json_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  auto result = diplomat::capi::Transaction_to_json(this->AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline const diplomat::capi::Transaction* Transaction::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::Transaction*>(this);
}

inline diplomat::capi::Transaction* Transaction::AsFFI() {
  return reinterpret_cast<diplomat::capi::Transaction*>(this);
}

inline const Transaction* Transaction::FromFFI(const diplomat::capi::Transaction* ptr) {
  return reinterpret_cast<const Transaction*>(ptr);
}

inline Transaction* Transaction::FromFFI(diplomat::capi::Transaction* ptr) {
  return reinterpret_cast<Transaction*>(ptr);
}

inline void Transaction::operator delete(void* ptr) {
  diplomat::capi::Transaction_destroy(reinterpret_cast<diplomat::capi::Transaction*>(ptr));
}


#endif // Transaction_HPP
