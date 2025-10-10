#ifndef TokenBalance_HPP
#define TokenBalance_HPP

#include "TokenBalance.d.hpp"

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

    void TokenBalance_account_address(const diplomat::capi::TokenBalance* self, diplomat::capi::DiplomatWrite* write);

    void TokenBalance_contract_address(const diplomat::capi::TokenBalance* self, diplomat::capi::DiplomatWrite* write);

    void TokenBalance_balance(const diplomat::capi::TokenBalance* self, diplomat::capi::DiplomatWrite* write);

    typedef struct TokenBalance_from_json_result {union {diplomat::capi::TokenBalance* ok; diplomat::capi::DojoError* err;}; bool is_ok;} TokenBalance_from_json_result;
    TokenBalance_from_json_result TokenBalance_from_json(diplomat::capi::DiplomatStringView json);

    typedef struct TokenBalance_to_json_result {union { diplomat::capi::DojoError* err;}; bool is_ok;} TokenBalance_to_json_result;
    TokenBalance_to_json_result TokenBalance_to_json(const diplomat::capi::TokenBalance* self, diplomat::capi::DiplomatWrite* write);

    void TokenBalance_destroy(TokenBalance* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::string TokenBalance::account_address() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::TokenBalance_account_address(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void TokenBalance::account_address_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::TokenBalance_account_address(this->AsFFI(),
    &write);
}

inline std::string TokenBalance::contract_address() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::TokenBalance_contract_address(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void TokenBalance::contract_address_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::TokenBalance_contract_address(this->AsFFI(),
    &write);
}

inline std::string TokenBalance::balance() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::TokenBalance_balance(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void TokenBalance::balance_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::TokenBalance_balance(this->AsFFI(),
    &write);
}

inline diplomat::result<std::unique_ptr<TokenBalance>, std::unique_ptr<DojoError>> TokenBalance::from_json(std::string_view json) {
  auto result = diplomat::capi::TokenBalance_from_json({json.data(), json.size()});
  return result.is_ok ? diplomat::result<std::unique_ptr<TokenBalance>, std::unique_ptr<DojoError>>(diplomat::Ok<std::unique_ptr<TokenBalance>>(std::unique_ptr<TokenBalance>(TokenBalance::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<TokenBalance>, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<DojoError>> TokenBalance::to_json() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::TokenBalance_to_json(this->AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, std::unique_ptr<DojoError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> TokenBalance::to_json_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  auto result = diplomat::capi::TokenBalance_to_json(this->AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline const diplomat::capi::TokenBalance* TokenBalance::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::TokenBalance*>(this);
}

inline diplomat::capi::TokenBalance* TokenBalance::AsFFI() {
  return reinterpret_cast<diplomat::capi::TokenBalance*>(this);
}

inline const TokenBalance* TokenBalance::FromFFI(const diplomat::capi::TokenBalance* ptr) {
  return reinterpret_cast<const TokenBalance*>(ptr);
}

inline TokenBalance* TokenBalance::FromFFI(diplomat::capi::TokenBalance* ptr) {
  return reinterpret_cast<TokenBalance*>(ptr);
}

inline void TokenBalance::operator delete(void* ptr) {
  diplomat::capi::TokenBalance_destroy(reinterpret_cast<diplomat::capi::TokenBalance*>(ptr));
}


#endif // TokenBalance_HPP
