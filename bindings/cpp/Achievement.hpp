#ifndef Achievement_HPP
#define Achievement_HPP

#include "Achievement.d.hpp"

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

    void Achievement_id(const diplomat::capi::Achievement* self, diplomat::capi::DiplomatWrite* write);

    void Achievement_world_address(const diplomat::capi::Achievement* self, diplomat::capi::DiplomatWrite* write);

    void Achievement_namespace(const diplomat::capi::Achievement* self, diplomat::capi::DiplomatWrite* write);

    void Achievement_title(const diplomat::capi::Achievement* self, diplomat::capi::DiplomatWrite* write);

    void Achievement_description(const diplomat::capi::Achievement* self, diplomat::capi::DiplomatWrite* write);

    bool Achievement_hidden(const diplomat::capi::Achievement* self);

    void Achievement_icon(const diplomat::capi::Achievement* self, diplomat::capi::DiplomatWrite* write);

    uint32_t Achievement_points(const diplomat::capi::Achievement* self);

    typedef struct Achievement_from_json_result {union {diplomat::capi::Achievement* ok; diplomat::capi::DojoError* err;}; bool is_ok;} Achievement_from_json_result;
    Achievement_from_json_result Achievement_from_json(diplomat::capi::DiplomatStringView json);

    typedef struct Achievement_to_json_result {union { diplomat::capi::DojoError* err;}; bool is_ok;} Achievement_to_json_result;
    Achievement_to_json_result Achievement_to_json(const diplomat::capi::Achievement* self, diplomat::capi::DiplomatWrite* write);

    void Achievement_destroy(Achievement* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::string Achievement::id() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::Achievement_id(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void Achievement::id_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::Achievement_id(this->AsFFI(),
    &write);
}

inline std::string Achievement::world_address() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::Achievement_world_address(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void Achievement::world_address_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::Achievement_world_address(this->AsFFI(),
    &write);
}

inline std::string Achievement::namespace_() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::Achievement_namespace(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void Achievement::namespace__write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::Achievement_namespace(this->AsFFI(),
    &write);
}

inline std::string Achievement::title() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::Achievement_title(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void Achievement::title_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::Achievement_title(this->AsFFI(),
    &write);
}

inline std::string Achievement::description() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::Achievement_description(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void Achievement::description_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::Achievement_description(this->AsFFI(),
    &write);
}

inline bool Achievement::hidden() const {
  auto result = diplomat::capi::Achievement_hidden(this->AsFFI());
  return result;
}

inline std::string Achievement::icon() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::Achievement_icon(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void Achievement::icon_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::Achievement_icon(this->AsFFI(),
    &write);
}

inline uint32_t Achievement::points() const {
  auto result = diplomat::capi::Achievement_points(this->AsFFI());
  return result;
}

inline diplomat::result<std::unique_ptr<Achievement>, std::unique_ptr<DojoError>> Achievement::from_json(std::string_view json) {
  auto result = diplomat::capi::Achievement_from_json({json.data(), json.size()});
  return result.is_ok ? diplomat::result<std::unique_ptr<Achievement>, std::unique_ptr<DojoError>>(diplomat::Ok<std::unique_ptr<Achievement>>(std::unique_ptr<Achievement>(Achievement::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<Achievement>, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<DojoError>> Achievement::to_json() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::Achievement_to_json(this->AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, std::unique_ptr<DojoError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> Achievement::to_json_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  auto result = diplomat::capi::Achievement_to_json(this->AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline const diplomat::capi::Achievement* Achievement::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::Achievement*>(this);
}

inline diplomat::capi::Achievement* Achievement::AsFFI() {
  return reinterpret_cast<diplomat::capi::Achievement*>(this);
}

inline const Achievement* Achievement::FromFFI(const diplomat::capi::Achievement* ptr) {
  return reinterpret_cast<const Achievement*>(ptr);
}

inline Achievement* Achievement::FromFFI(diplomat::capi::Achievement* ptr) {
  return reinterpret_cast<Achievement*>(ptr);
}

inline void Achievement::operator delete(void* ptr) {
  diplomat::capi::Achievement_destroy(reinterpret_cast<diplomat::capi::Achievement*>(ptr));
}


#endif // Achievement_HPP
