#ifndef Account_HPP
#define Account_HPP

#include "Account.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "CallList.hpp"
#include "DojoError.hpp"
#include "FieldElement.hpp"
#include "Provider.hpp"
#include "SigningKey.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {

    diplomat::capi::Account* Account_new(const diplomat::capi::Provider* provider, const diplomat::capi::SigningKey* signer, const diplomat::capi::FieldElement* address, const diplomat::capi::FieldElement* chain_id);

    void Account_address(const diplomat::capi::Account* self, diplomat::capi::DiplomatWrite* write);

    void Account_chain_id(const diplomat::capi::Account* self, diplomat::capi::DiplomatWrite* write);

    typedef struct Account_execute_result {union { diplomat::capi::DojoError* err;}; bool is_ok;} Account_execute_result;
    Account_execute_result Account_execute(const diplomat::capi::Account* self, const diplomat::capi::CallList* calls, diplomat::capi::DiplomatWrite* write);

    typedef struct Account_nonce_result {union {uint64_t ok; diplomat::capi::DojoError* err;}; bool is_ok;} Account_nonce_result;
    Account_nonce_result Account_nonce(const diplomat::capi::Account* self);

    void Account_destroy(Account* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<Account> Account::new_(const Provider& provider, const SigningKey& signer, const FieldElement& address, const FieldElement& chain_id) {
  auto result = diplomat::capi::Account_new(provider.AsFFI(),
    signer.AsFFI(),
    address.AsFFI(),
    chain_id.AsFFI());
  return std::unique_ptr<Account>(Account::FromFFI(result));
}

inline std::string Account::address() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::Account_address(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void Account::address_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::Account_address(this->AsFFI(),
    &write);
}

inline std::string Account::chain_id() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::Account_chain_id(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void Account::chain_id_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::Account_chain_id(this->AsFFI(),
    &write);
}

inline diplomat::result<std::string, std::unique_ptr<DojoError>> Account::execute(const CallList& calls) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::Account_execute(this->AsFFI(),
    calls.AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, std::unique_ptr<DojoError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> Account::execute_write(const CallList& calls, W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  auto result = diplomat::capi::Account_execute(this->AsFFI(),
    calls.AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline diplomat::result<uint64_t, std::unique_ptr<DojoError>> Account::nonce() const {
  auto result = diplomat::capi::Account_nonce(this->AsFFI());
  return result.is_ok ? diplomat::result<uint64_t, std::unique_ptr<DojoError>>(diplomat::Ok<uint64_t>(result.ok)) : diplomat::result<uint64_t, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline const diplomat::capi::Account* Account::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::Account*>(this);
}

inline diplomat::capi::Account* Account::AsFFI() {
  return reinterpret_cast<diplomat::capi::Account*>(this);
}

inline const Account* Account::FromFFI(const diplomat::capi::Account* ptr) {
  return reinterpret_cast<const Account*>(ptr);
}

inline Account* Account::FromFFI(diplomat::capi::Account* ptr) {
  return reinterpret_cast<Account*>(ptr);
}

inline void Account::operator delete(void* ptr) {
  diplomat::capi::Account_destroy(reinterpret_cast<diplomat::capi::Account*>(ptr));
}


#endif // Account_HPP
