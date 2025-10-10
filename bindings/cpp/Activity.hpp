#ifndef Activity_HPP
#define Activity_HPP

#include "Activity.d.hpp"

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

    void Activity_id(const diplomat::capi::Activity* self, diplomat::capi::DiplomatWrite* write);

    void Activity_world_address(const diplomat::capi::Activity* self, diplomat::capi::DiplomatWrite* write);

    void Activity_namespace(const diplomat::capi::Activity* self, diplomat::capi::DiplomatWrite* write);

    void Activity_caller_address(const diplomat::capi::Activity* self, diplomat::capi::DiplomatWrite* write);

    uint64_t Activity_session_start(const diplomat::capi::Activity* self);

    uint64_t Activity_session_end(const diplomat::capi::Activity* self);

    uint32_t Activity_action_count(const diplomat::capi::Activity* self);

    uint64_t Activity_updated_at(const diplomat::capi::Activity* self);

    typedef struct Activity_from_json_result {union {diplomat::capi::Activity* ok; diplomat::capi::DojoError* err;}; bool is_ok;} Activity_from_json_result;
    Activity_from_json_result Activity_from_json(diplomat::capi::DiplomatStringView json);

    typedef struct Activity_to_json_result {union { diplomat::capi::DojoError* err;}; bool is_ok;} Activity_to_json_result;
    Activity_to_json_result Activity_to_json(const diplomat::capi::Activity* self, diplomat::capi::DiplomatWrite* write);

    void Activity_destroy(Activity* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::string Activity::id() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::Activity_id(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void Activity::id_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::Activity_id(this->AsFFI(),
    &write);
}

inline std::string Activity::world_address() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::Activity_world_address(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void Activity::world_address_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::Activity_world_address(this->AsFFI(),
    &write);
}

inline std::string Activity::namespace_() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::Activity_namespace(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void Activity::namespace__write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::Activity_namespace(this->AsFFI(),
    &write);
}

inline std::string Activity::caller_address() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::Activity_caller_address(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void Activity::caller_address_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::Activity_caller_address(this->AsFFI(),
    &write);
}

inline uint64_t Activity::session_start() const {
  auto result = diplomat::capi::Activity_session_start(this->AsFFI());
  return result;
}

inline uint64_t Activity::session_end() const {
  auto result = diplomat::capi::Activity_session_end(this->AsFFI());
  return result;
}

inline uint32_t Activity::action_count() const {
  auto result = diplomat::capi::Activity_action_count(this->AsFFI());
  return result;
}

inline uint64_t Activity::updated_at() const {
  auto result = diplomat::capi::Activity_updated_at(this->AsFFI());
  return result;
}

inline diplomat::result<std::unique_ptr<Activity>, std::unique_ptr<DojoError>> Activity::from_json(std::string_view json) {
  auto result = diplomat::capi::Activity_from_json({json.data(), json.size()});
  return result.is_ok ? diplomat::result<std::unique_ptr<Activity>, std::unique_ptr<DojoError>>(diplomat::Ok<std::unique_ptr<Activity>>(std::unique_ptr<Activity>(Activity::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<Activity>, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<DojoError>> Activity::to_json() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::Activity_to_json(this->AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, std::unique_ptr<DojoError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> Activity::to_json_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  auto result = diplomat::capi::Activity_to_json(this->AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline const diplomat::capi::Activity* Activity::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::Activity*>(this);
}

inline diplomat::capi::Activity* Activity::AsFFI() {
  return reinterpret_cast<diplomat::capi::Activity*>(this);
}

inline const Activity* Activity::FromFFI(const diplomat::capi::Activity* ptr) {
  return reinterpret_cast<const Activity*>(ptr);
}

inline Activity* Activity::FromFFI(diplomat::capi::Activity* ptr) {
  return reinterpret_cast<Activity*>(ptr);
}

inline void Activity::operator delete(void* ptr) {
  diplomat::capi::Activity_destroy(reinterpret_cast<diplomat::capi::Activity*>(ptr));
}


#endif // Activity_HPP
