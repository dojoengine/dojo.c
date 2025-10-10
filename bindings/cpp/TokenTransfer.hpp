#ifndef TokenTransfer_HPP
#define TokenTransfer_HPP

#include "TokenTransfer.d.hpp"

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

    void TokenTransfer_from_address(const diplomat::capi::TokenTransfer* self, diplomat::capi::DiplomatWrite* write);

    void TokenTransfer_to_address(const diplomat::capi::TokenTransfer* self, diplomat::capi::DiplomatWrite* write);

    void TokenTransfer_contract_address(const diplomat::capi::TokenTransfer* self, diplomat::capi::DiplomatWrite* write);

    void TokenTransfer_amount(const diplomat::capi::TokenTransfer* self, diplomat::capi::DiplomatWrite* write);

    uint64_t TokenTransfer_executed_at(const diplomat::capi::TokenTransfer* self);

    typedef struct TokenTransfer_from_json_result {union {diplomat::capi::TokenTransfer* ok; diplomat::capi::DojoError* err;}; bool is_ok;} TokenTransfer_from_json_result;
    TokenTransfer_from_json_result TokenTransfer_from_json(diplomat::capi::DiplomatStringView json);

    typedef struct TokenTransfer_to_json_result {union { diplomat::capi::DojoError* err;}; bool is_ok;} TokenTransfer_to_json_result;
    TokenTransfer_to_json_result TokenTransfer_to_json(const diplomat::capi::TokenTransfer* self, diplomat::capi::DiplomatWrite* write);

    void TokenTransfer_destroy(TokenTransfer* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::string TokenTransfer::from_address() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::TokenTransfer_from_address(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void TokenTransfer::from_address_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::TokenTransfer_from_address(this->AsFFI(),
    &write);
}

inline std::string TokenTransfer::to_address() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::TokenTransfer_to_address(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void TokenTransfer::to_address_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::TokenTransfer_to_address(this->AsFFI(),
    &write);
}

inline std::string TokenTransfer::contract_address() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::TokenTransfer_contract_address(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void TokenTransfer::contract_address_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::TokenTransfer_contract_address(this->AsFFI(),
    &write);
}

inline std::string TokenTransfer::amount() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::TokenTransfer_amount(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void TokenTransfer::amount_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::TokenTransfer_amount(this->AsFFI(),
    &write);
}

inline uint64_t TokenTransfer::executed_at() const {
  auto result = diplomat::capi::TokenTransfer_executed_at(this->AsFFI());
  return result;
}

inline diplomat::result<std::unique_ptr<TokenTransfer>, std::unique_ptr<DojoError>> TokenTransfer::from_json(std::string_view json) {
  auto result = diplomat::capi::TokenTransfer_from_json({json.data(), json.size()});
  return result.is_ok ? diplomat::result<std::unique_ptr<TokenTransfer>, std::unique_ptr<DojoError>>(diplomat::Ok<std::unique_ptr<TokenTransfer>>(std::unique_ptr<TokenTransfer>(TokenTransfer::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<TokenTransfer>, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<DojoError>> TokenTransfer::to_json() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::TokenTransfer_to_json(this->AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, std::unique_ptr<DojoError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> TokenTransfer::to_json_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  auto result = diplomat::capi::TokenTransfer_to_json(this->AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline const diplomat::capi::TokenTransfer* TokenTransfer::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::TokenTransfer*>(this);
}

inline diplomat::capi::TokenTransfer* TokenTransfer::AsFFI() {
  return reinterpret_cast<diplomat::capi::TokenTransfer*>(this);
}

inline const TokenTransfer* TokenTransfer::FromFFI(const diplomat::capi::TokenTransfer* ptr) {
  return reinterpret_cast<const TokenTransfer*>(ptr);
}

inline TokenTransfer* TokenTransfer::FromFFI(diplomat::capi::TokenTransfer* ptr) {
  return reinterpret_cast<TokenTransfer*>(ptr);
}

inline void TokenTransfer::operator delete(void* ptr) {
  diplomat::capi::TokenTransfer_destroy(reinterpret_cast<diplomat::capi::TokenTransfer*>(ptr));
}


#endif // TokenTransfer_HPP
