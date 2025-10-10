#ifndef Contract_HPP
#define Contract_HPP

#include "Contract.d.hpp"

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

    void Contract_contract_address(const diplomat::capi::Contract* self, diplomat::capi::DiplomatWrite* write);

    void Contract_contract_type(const diplomat::capi::Contract* self, diplomat::capi::DiplomatWrite* write);

    uint64_t Contract_head(const diplomat::capi::Contract* self);

    uint64_t Contract_tps(const diplomat::capi::Contract* self);

    uint64_t Contract_created_at(const diplomat::capi::Contract* self);

    uint64_t Contract_updated_at(const diplomat::capi::Contract* self);

    typedef struct Contract_from_json_result {union {diplomat::capi::Contract* ok; diplomat::capi::DojoError* err;}; bool is_ok;} Contract_from_json_result;
    Contract_from_json_result Contract_from_json(diplomat::capi::DiplomatStringView json);

    typedef struct Contract_to_json_result {union { diplomat::capi::DojoError* err;}; bool is_ok;} Contract_to_json_result;
    Contract_to_json_result Contract_to_json(const diplomat::capi::Contract* self, diplomat::capi::DiplomatWrite* write);

    void Contract_destroy(Contract* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::string Contract::contract_address() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::Contract_contract_address(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void Contract::contract_address_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::Contract_contract_address(this->AsFFI(),
    &write);
}

inline std::string Contract::contract_type() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::Contract_contract_type(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void Contract::contract_type_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::Contract_contract_type(this->AsFFI(),
    &write);
}

inline uint64_t Contract::head() const {
  auto result = diplomat::capi::Contract_head(this->AsFFI());
  return result;
}

inline uint64_t Contract::tps() const {
  auto result = diplomat::capi::Contract_tps(this->AsFFI());
  return result;
}

inline uint64_t Contract::created_at() const {
  auto result = diplomat::capi::Contract_created_at(this->AsFFI());
  return result;
}

inline uint64_t Contract::updated_at() const {
  auto result = diplomat::capi::Contract_updated_at(this->AsFFI());
  return result;
}

inline diplomat::result<std::unique_ptr<Contract>, std::unique_ptr<DojoError>> Contract::from_json(std::string_view json) {
  auto result = diplomat::capi::Contract_from_json({json.data(), json.size()});
  return result.is_ok ? diplomat::result<std::unique_ptr<Contract>, std::unique_ptr<DojoError>>(diplomat::Ok<std::unique_ptr<Contract>>(std::unique_ptr<Contract>(Contract::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<Contract>, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<DojoError>> Contract::to_json() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::Contract_to_json(this->AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, std::unique_ptr<DojoError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> Contract::to_json_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  auto result = diplomat::capi::Contract_to_json(this->AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline const diplomat::capi::Contract* Contract::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::Contract*>(this);
}

inline diplomat::capi::Contract* Contract::AsFFI() {
  return reinterpret_cast<diplomat::capi::Contract*>(this);
}

inline const Contract* Contract::FromFFI(const diplomat::capi::Contract* ptr) {
  return reinterpret_cast<const Contract*>(ptr);
}

inline Contract* Contract::FromFFI(diplomat::capi::Contract* ptr) {
  return reinterpret_cast<Contract*>(ptr);
}

inline void Contract::operator delete(void* ptr) {
  diplomat::capi::Contract_destroy(reinterpret_cast<diplomat::capi::Contract*>(ptr));
}


#endif // Contract_HPP
