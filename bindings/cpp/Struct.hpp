#ifndef Struct_HPP
#define Struct_HPP

#include "Struct.d.hpp"

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

    void Struct_name(const diplomat::capi::Struct* self, diplomat::capi::DiplomatWrite* write);

    uint32_t Struct_children_count(const diplomat::capi::Struct* self);

    typedef struct Struct_from_json_result {union {diplomat::capi::Struct* ok; diplomat::capi::DojoError* err;}; bool is_ok;} Struct_from_json_result;
    Struct_from_json_result Struct_from_json(diplomat::capi::DiplomatStringView json);

    typedef struct Struct_to_json_result {union { diplomat::capi::DojoError* err;}; bool is_ok;} Struct_to_json_result;
    Struct_to_json_result Struct_to_json(const diplomat::capi::Struct* self, diplomat::capi::DiplomatWrite* write);

    void Struct_destroy(Struct* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::string Struct::name() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::Struct_name(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void Struct::name_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::Struct_name(this->AsFFI(),
    &write);
}

inline uint32_t Struct::children_count() const {
  auto result = diplomat::capi::Struct_children_count(this->AsFFI());
  return result;
}

inline diplomat::result<std::unique_ptr<Struct>, std::unique_ptr<DojoError>> Struct::from_json(std::string_view json) {
  auto result = diplomat::capi::Struct_from_json({json.data(), json.size()});
  return result.is_ok ? diplomat::result<std::unique_ptr<Struct>, std::unique_ptr<DojoError>>(diplomat::Ok<std::unique_ptr<Struct>>(std::unique_ptr<Struct>(Struct::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<Struct>, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<DojoError>> Struct::to_json() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::Struct_to_json(this->AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, std::unique_ptr<DojoError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> Struct::to_json_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  auto result = diplomat::capi::Struct_to_json(this->AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline const diplomat::capi::Struct* Struct::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::Struct*>(this);
}

inline diplomat::capi::Struct* Struct::AsFFI() {
  return reinterpret_cast<diplomat::capi::Struct*>(this);
}

inline const Struct* Struct::FromFFI(const diplomat::capi::Struct* ptr) {
  return reinterpret_cast<const Struct*>(ptr);
}

inline Struct* Struct::FromFFI(diplomat::capi::Struct* ptr) {
  return reinterpret_cast<Struct*>(ptr);
}

inline void Struct::operator delete(void* ptr) {
  diplomat::capi::Struct_destroy(reinterpret_cast<diplomat::capi::Struct*>(ptr));
}


#endif // Struct_HPP
