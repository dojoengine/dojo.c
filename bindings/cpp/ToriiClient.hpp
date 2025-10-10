#ifndef ToriiClient_HPP
#define ToriiClient_HPP

#include "ToriiClient.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "DojoError.hpp"
#include "FieldElement.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {

    typedef struct ToriiClient_new_result {union {diplomat::capi::ToriiClient* ok; diplomat::capi::DojoError* err;}; bool is_ok;} ToriiClient_new_result;
    ToriiClient_new_result ToriiClient_new(diplomat::capi::DiplomatStringView torii_url);

    typedef struct ToriiClient_info_result {union { diplomat::capi::DojoError* err;}; bool is_ok;} ToriiClient_info_result;
    ToriiClient_info_result ToriiClient_info(const diplomat::capi::ToriiClient* self, diplomat::capi::DiplomatWrite* write);

    typedef struct ToriiClient_publish_message_result {union { diplomat::capi::DojoError* err;}; bool is_ok;} ToriiClient_publish_message_result;
    ToriiClient_publish_message_result ToriiClient_publish_message(const diplomat::capi::ToriiClient* self, diplomat::capi::DiplomatStringView message_json, const diplomat::capi::FieldElement* signature_r, const diplomat::capi::FieldElement* signature_s, const diplomat::capi::FieldElement* world_address, diplomat::capi::DiplomatWrite* write);

    void ToriiClient_destroy(ToriiClient* self);

    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<ToriiClient>, std::unique_ptr<DojoError>> ToriiClient::new_(std::string_view torii_url) {
  auto result = diplomat::capi::ToriiClient_new({torii_url.data(), torii_url.size()});
  return result.is_ok ? diplomat::result<std::unique_ptr<ToriiClient>, std::unique_ptr<DojoError>>(diplomat::Ok<std::unique_ptr<ToriiClient>>(std::unique_ptr<ToriiClient>(ToriiClient::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ToriiClient>, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<DojoError>> ToriiClient::info() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::ToriiClient_info(this->AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, std::unique_ptr<DojoError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> ToriiClient::info_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  auto result = diplomat::capi::ToriiClient_info(this->AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<DojoError>> ToriiClient::publish_message(std::string_view message_json, const FieldElement& signature_r, const FieldElement& signature_s, const FieldElement& world_address) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::ToriiClient_publish_message(this->AsFFI(),
    {message_json.data(), message_json.size()},
    signature_r.AsFFI(),
    signature_s.AsFFI(),
    world_address.AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, std::unique_ptr<DojoError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> ToriiClient::publish_message_write(std::string_view message_json, const FieldElement& signature_r, const FieldElement& signature_s, const FieldElement& world_address, W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  auto result = diplomat::capi::ToriiClient_publish_message(this->AsFFI(),
    {message_json.data(), message_json.size()},
    signature_r.AsFFI(),
    signature_s.AsFFI(),
    world_address.AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline const diplomat::capi::ToriiClient* ToriiClient::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::ToriiClient*>(this);
}

inline diplomat::capi::ToriiClient* ToriiClient::AsFFI() {
  return reinterpret_cast<diplomat::capi::ToriiClient*>(this);
}

inline const ToriiClient* ToriiClient::FromFFI(const diplomat::capi::ToriiClient* ptr) {
  return reinterpret_cast<const ToriiClient*>(ptr);
}

inline ToriiClient* ToriiClient::FromFFI(diplomat::capi::ToriiClient* ptr) {
  return reinterpret_cast<ToriiClient*>(ptr);
}

inline void ToriiClient::operator delete(void* ptr) {
  diplomat::capi::ToriiClient_destroy(reinterpret_cast<diplomat::capi::ToriiClient*>(ptr));
}


#endif // ToriiClient_HPP
