#ifndef Primitive_HPP
#define Primitive_HPP

#include "Primitive.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "DojoError.hpp"
#include "PrimitiveType.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {

    diplomat::capi::PrimitiveType Primitive_primitive_type(const diplomat::capi::Primitive* self);

    typedef struct Primitive_from_json_result {union {diplomat::capi::Primitive* ok; diplomat::capi::DojoError* err;}; bool is_ok;} Primitive_from_json_result;
    Primitive_from_json_result Primitive_from_json(diplomat::capi::DiplomatStringView json);

    typedef struct Primitive_to_json_result {union { diplomat::capi::DojoError* err;}; bool is_ok;} Primitive_to_json_result;
    Primitive_to_json_result Primitive_to_json(const diplomat::capi::Primitive* self, diplomat::capi::DiplomatWrite* write);

    void Primitive_destroy(Primitive* self);

    } // extern "C"
} // namespace capi
} // namespace

inline PrimitiveType Primitive::primitive_type() const {
  auto result = diplomat::capi::Primitive_primitive_type(this->AsFFI());
  return PrimitiveType::FromFFI(result);
}

inline diplomat::result<std::unique_ptr<Primitive>, std::unique_ptr<DojoError>> Primitive::from_json(std::string_view json) {
  auto result = diplomat::capi::Primitive_from_json({json.data(), json.size()});
  return result.is_ok ? diplomat::result<std::unique_ptr<Primitive>, std::unique_ptr<DojoError>>(diplomat::Ok<std::unique_ptr<Primitive>>(std::unique_ptr<Primitive>(Primitive::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<Primitive>, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<DojoError>> Primitive::to_json() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::Primitive_to_json(this->AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, std::unique_ptr<DojoError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> Primitive::to_json_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  auto result = diplomat::capi::Primitive_to_json(this->AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline const diplomat::capi::Primitive* Primitive::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::Primitive*>(this);
}

inline diplomat::capi::Primitive* Primitive::AsFFI() {
  return reinterpret_cast<diplomat::capi::Primitive*>(this);
}

inline const Primitive* Primitive::FromFFI(const diplomat::capi::Primitive* ptr) {
  return reinterpret_cast<const Primitive*>(ptr);
}

inline Primitive* Primitive::FromFFI(diplomat::capi::Primitive* ptr) {
  return reinterpret_cast<Primitive*>(ptr);
}

inline void Primitive::operator delete(void* ptr) {
  diplomat::capi::Primitive_destroy(reinterpret_cast<diplomat::capi::Primitive*>(ptr));
}


#endif // Primitive_HPP
