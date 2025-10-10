#ifndef Token_HPP
#define Token_HPP

#include "Token.d.hpp"

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

    void Token_contract_address(const diplomat::capi::Token* self, diplomat::capi::DiplomatWrite* write);

    void Token_name(const diplomat::capi::Token* self, diplomat::capi::DiplomatWrite* write);

    void Token_symbol(const diplomat::capi::Token* self, diplomat::capi::DiplomatWrite* write);

    uint8_t Token_decimals(const diplomat::capi::Token* self);

    void Token_metadata(const diplomat::capi::Token* self, diplomat::capi::DiplomatWrite* write);

    typedef struct Token_from_json_result {union {diplomat::capi::Token* ok; diplomat::capi::DojoError* err;}; bool is_ok;} Token_from_json_result;
    Token_from_json_result Token_from_json(diplomat::capi::DiplomatStringView json);

    typedef struct Token_to_json_result {union { diplomat::capi::DojoError* err;}; bool is_ok;} Token_to_json_result;
    Token_to_json_result Token_to_json(const diplomat::capi::Token* self, diplomat::capi::DiplomatWrite* write);

    void Token_destroy(Token* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::string Token::contract_address() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::Token_contract_address(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void Token::contract_address_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::Token_contract_address(this->AsFFI(),
    &write);
}

inline std::string Token::name() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::Token_name(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void Token::name_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::Token_name(this->AsFFI(),
    &write);
}

inline std::string Token::symbol() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::Token_symbol(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void Token::symbol_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::Token_symbol(this->AsFFI(),
    &write);
}

inline uint8_t Token::decimals() const {
  auto result = diplomat::capi::Token_decimals(this->AsFFI());
  return result;
}

inline std::string Token::metadata() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::Token_metadata(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void Token::metadata_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::Token_metadata(this->AsFFI(),
    &write);
}

inline diplomat::result<std::unique_ptr<Token>, std::unique_ptr<DojoError>> Token::from_json(std::string_view json) {
  auto result = diplomat::capi::Token_from_json({json.data(), json.size()});
  return result.is_ok ? diplomat::result<std::unique_ptr<Token>, std::unique_ptr<DojoError>>(diplomat::Ok<std::unique_ptr<Token>>(std::unique_ptr<Token>(Token::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<Token>, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<DojoError>> Token::to_json() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::Token_to_json(this->AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, std::unique_ptr<DojoError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> Token::to_json_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  auto result = diplomat::capi::Token_to_json(this->AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline const diplomat::capi::Token* Token::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::Token*>(this);
}

inline diplomat::capi::Token* Token::AsFFI() {
  return reinterpret_cast<diplomat::capi::Token*>(this);
}

inline const Token* Token::FromFFI(const diplomat::capi::Token* ptr) {
  return reinterpret_cast<const Token*>(ptr);
}

inline Token* Token::FromFFI(diplomat::capi::Token* ptr) {
  return reinterpret_cast<Token*>(ptr);
}

inline void Token::operator delete(void* ptr) {
  diplomat::capi::Token_destroy(reinterpret_cast<diplomat::capi::Token*>(ptr));
}


#endif // Token_HPP
