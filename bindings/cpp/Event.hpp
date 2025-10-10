#ifndef Event_HPP
#define Event_HPP

#include "Event.d.hpp"

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

    void Event_keys(const diplomat::capi::Event* self, diplomat::capi::DiplomatWrite* write);

    void Event_data(const diplomat::capi::Event* self, diplomat::capi::DiplomatWrite* write);

    void Event_transaction_hash(const diplomat::capi::Event* self, diplomat::capi::DiplomatWrite* write);

    typedef struct Event_from_json_result {union {diplomat::capi::Event* ok; diplomat::capi::DojoError* err;}; bool is_ok;} Event_from_json_result;
    Event_from_json_result Event_from_json(diplomat::capi::DiplomatStringView json);

    typedef struct Event_to_json_result {union { diplomat::capi::DojoError* err;}; bool is_ok;} Event_to_json_result;
    Event_to_json_result Event_to_json(const diplomat::capi::Event* self, diplomat::capi::DiplomatWrite* write);

    void Event_destroy(Event* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::string Event::keys() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::Event_keys(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void Event::keys_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::Event_keys(this->AsFFI(),
    &write);
}

inline std::string Event::data() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::Event_data(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void Event::data_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::Event_data(this->AsFFI(),
    &write);
}

inline std::string Event::transaction_hash() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::Event_transaction_hash(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void Event::transaction_hash_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::Event_transaction_hash(this->AsFFI(),
    &write);
}

inline diplomat::result<std::unique_ptr<Event>, std::unique_ptr<DojoError>> Event::from_json(std::string_view json) {
  auto result = diplomat::capi::Event_from_json({json.data(), json.size()});
  return result.is_ok ? diplomat::result<std::unique_ptr<Event>, std::unique_ptr<DojoError>>(diplomat::Ok<std::unique_ptr<Event>>(std::unique_ptr<Event>(Event::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<Event>, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<DojoError>> Event::to_json() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::Event_to_json(this->AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, std::unique_ptr<DojoError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> Event::to_json_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  auto result = diplomat::capi::Event_to_json(this->AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline const diplomat::capi::Event* Event::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::Event*>(this);
}

inline diplomat::capi::Event* Event::AsFFI() {
  return reinterpret_cast<diplomat::capi::Event*>(this);
}

inline const Event* Event::FromFFI(const diplomat::capi::Event* ptr) {
  return reinterpret_cast<const Event*>(ptr);
}

inline Event* Event::FromFFI(diplomat::capi::Event* ptr) {
  return reinterpret_cast<Event*>(ptr);
}

inline void Event::operator delete(void* ptr) {
  diplomat::capi::Event_destroy(reinterpret_cast<diplomat::capi::Event*>(ptr));
}


#endif // Event_HPP
