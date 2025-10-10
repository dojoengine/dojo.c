#ifndef TokenContract_HPP
#define TokenContract_HPP

#include "TokenContract.d.hpp"

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

    void TokenContract_contract_address(const diplomat::capi::TokenContract* self, diplomat::capi::DiplomatWrite* write);

    void TokenContract_contract_type(const diplomat::capi::TokenContract* self, diplomat::capi::DiplomatWrite* write);

    typedef struct TokenContract_from_json_result {union {diplomat::capi::TokenContract* ok; diplomat::capi::DojoError* err;}; bool is_ok;} TokenContract_from_json_result;
    TokenContract_from_json_result TokenContract_from_json(diplomat::capi::DiplomatStringView json);

    typedef struct TokenContract_to_json_result {union { diplomat::capi::DojoError* err;}; bool is_ok;} TokenContract_to_json_result;
    TokenContract_to_json_result TokenContract_to_json(const diplomat::capi::TokenContract* self, diplomat::capi::DiplomatWrite* write);

    void TokenContract_destroy(TokenContract* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::string TokenContract::contract_address() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::TokenContract_contract_address(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void TokenContract::contract_address_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::TokenContract_contract_address(this->AsFFI(),
    &write);
}

inline std::string TokenContract::contract_type() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::TokenContract_contract_type(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void TokenContract::contract_type_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::TokenContract_contract_type(this->AsFFI(),
    &write);
}

inline diplomat::result<std::unique_ptr<TokenContract>, std::unique_ptr<DojoError>> TokenContract::from_json(std::string_view json) {
  auto result = diplomat::capi::TokenContract_from_json({json.data(), json.size()});
  return result.is_ok ? diplomat::result<std::unique_ptr<TokenContract>, std::unique_ptr<DojoError>>(diplomat::Ok<std::unique_ptr<TokenContract>>(std::unique_ptr<TokenContract>(TokenContract::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<TokenContract>, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<DojoError>> TokenContract::to_json() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::TokenContract_to_json(this->AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, std::unique_ptr<DojoError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> TokenContract::to_json_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  auto result = diplomat::capi::TokenContract_to_json(this->AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline const diplomat::capi::TokenContract* TokenContract::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::TokenContract*>(this);
}

inline diplomat::capi::TokenContract* TokenContract::AsFFI() {
  return reinterpret_cast<diplomat::capi::TokenContract*>(this);
}

inline const TokenContract* TokenContract::FromFFI(const diplomat::capi::TokenContract* ptr) {
  return reinterpret_cast<const TokenContract*>(ptr);
}

inline TokenContract* TokenContract::FromFFI(diplomat::capi::TokenContract* ptr) {
  return reinterpret_cast<TokenContract*>(ptr);
}

inline void TokenContract::operator delete(void* ptr) {
  diplomat::capi::TokenContract_destroy(reinterpret_cast<diplomat::capi::TokenContract*>(ptr));
}


#endif // TokenContract_HPP
