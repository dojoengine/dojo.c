#ifndef Provider_HPP
#define Provider_HPP

#include "Provider.d.hpp"

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

    typedef struct Provider_new_result {union {diplomat::capi::Provider* ok; diplomat::capi::DojoError* err;}; bool is_ok;} Provider_new_result;
    Provider_new_result Provider_new(diplomat::capi::DiplomatStringView rpc_url);

    typedef struct Provider_chain_id_result {union { diplomat::capi::DojoError* err;}; bool is_ok;} Provider_chain_id_result;
    Provider_chain_id_result Provider_chain_id(const diplomat::capi::Provider* self, diplomat::capi::DiplomatWrite* write);

    typedef struct Provider_block_number_result {union {uint64_t ok; diplomat::capi::DojoError* err;}; bool is_ok;} Provider_block_number_result;
    Provider_block_number_result Provider_block_number(const diplomat::capi::Provider* self);

    void Provider_destroy(Provider* self);

    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<Provider>, std::unique_ptr<DojoError>> Provider::new_(std::string_view rpc_url) {
  auto result = diplomat::capi::Provider_new({rpc_url.data(), rpc_url.size()});
  return result.is_ok ? diplomat::result<std::unique_ptr<Provider>, std::unique_ptr<DojoError>>(diplomat::Ok<std::unique_ptr<Provider>>(std::unique_ptr<Provider>(Provider::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<Provider>, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<DojoError>> Provider::chain_id() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::Provider_chain_id(this->AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, std::unique_ptr<DojoError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> Provider::chain_id_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  auto result = diplomat::capi::Provider_chain_id(this->AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline diplomat::result<uint64_t, std::unique_ptr<DojoError>> Provider::block_number() const {
  auto result = diplomat::capi::Provider_block_number(this->AsFFI());
  return result.is_ok ? diplomat::result<uint64_t, std::unique_ptr<DojoError>>(diplomat::Ok<uint64_t>(result.ok)) : diplomat::result<uint64_t, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline const diplomat::capi::Provider* Provider::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::Provider*>(this);
}

inline diplomat::capi::Provider* Provider::AsFFI() {
  return reinterpret_cast<diplomat::capi::Provider*>(this);
}

inline const Provider* Provider::FromFFI(const diplomat::capi::Provider* ptr) {
  return reinterpret_cast<const Provider*>(ptr);
}

inline Provider* Provider::FromFFI(diplomat::capi::Provider* ptr) {
  return reinterpret_cast<Provider*>(ptr);
}

inline void Provider::operator delete(void* ptr) {
  diplomat::capi::Provider_destroy(reinterpret_cast<diplomat::capi::Provider*>(ptr));
}


#endif // Provider_HPP
