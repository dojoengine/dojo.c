#ifndef Enum_HPP
#define Enum_HPP

#include "Enum.d.hpp"

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

    void Enum_name(const diplomat::capi::Enum* self, diplomat::capi::DiplomatWrite* write);

    uint8_t Enum_option(const diplomat::capi::Enum* self);

    uint32_t Enum_options_count(const diplomat::capi::Enum* self);

    typedef struct Enum_from_json_result {union {diplomat::capi::Enum* ok; diplomat::capi::DojoError* err;}; bool is_ok;} Enum_from_json_result;
    Enum_from_json_result Enum_from_json(diplomat::capi::DiplomatStringView json);

    typedef struct Enum_to_json_result {union { diplomat::capi::DojoError* err;}; bool is_ok;} Enum_to_json_result;
    Enum_to_json_result Enum_to_json(const diplomat::capi::Enum* self, diplomat::capi::DiplomatWrite* write);

    void Enum_destroy(Enum* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::string Enum::name() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::Enum_name(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void Enum::name_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::Enum_name(this->AsFFI(),
    &write);
}

inline uint8_t Enum::option() const {
  auto result = diplomat::capi::Enum_option(this->AsFFI());
  return result;
}

inline uint32_t Enum::options_count() const {
  auto result = diplomat::capi::Enum_options_count(this->AsFFI());
  return result;
}

inline diplomat::result<std::unique_ptr<Enum>, std::unique_ptr<DojoError>> Enum::from_json(std::string_view json) {
  auto result = diplomat::capi::Enum_from_json({json.data(), json.size()});
  return result.is_ok ? diplomat::result<std::unique_ptr<Enum>, std::unique_ptr<DojoError>>(diplomat::Ok<std::unique_ptr<Enum>>(std::unique_ptr<Enum>(Enum::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<Enum>, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<DojoError>> Enum::to_json() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::Enum_to_json(this->AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, std::unique_ptr<DojoError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> Enum::to_json_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  auto result = diplomat::capi::Enum_to_json(this->AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline const diplomat::capi::Enum* Enum::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::Enum*>(this);
}

inline diplomat::capi::Enum* Enum::AsFFI() {
  return reinterpret_cast<diplomat::capi::Enum*>(this);
}

inline const Enum* Enum::FromFFI(const diplomat::capi::Enum* ptr) {
  return reinterpret_cast<const Enum*>(ptr);
}

inline Enum* Enum::FromFFI(diplomat::capi::Enum* ptr) {
  return reinterpret_cast<Enum*>(ptr);
}

inline void Enum::operator delete(void* ptr) {
  diplomat::capi::Enum_destroy(reinterpret_cast<diplomat::capi::Enum*>(ptr));
}


#endif // Enum_HPP
