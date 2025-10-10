#ifndef TypedData_HPP
#define TypedData_HPP

#include "TypedData.d.hpp"

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

    typedef struct TypedData_new_from_json_result {union {diplomat::capi::TypedData* ok; diplomat::capi::DojoError* err;}; bool is_ok;} TypedData_new_from_json_result;
    TypedData_new_from_json_result TypedData_new_from_json(diplomat::capi::DiplomatStringView json);

    void TypedData_destroy(TypedData* self);

    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<TypedData>, std::unique_ptr<DojoError>> TypedData::new_from_json(std::string_view json) {
  auto result = diplomat::capi::TypedData_new_from_json({json.data(), json.size()});
  return result.is_ok ? diplomat::result<std::unique_ptr<TypedData>, std::unique_ptr<DojoError>>(diplomat::Ok<std::unique_ptr<TypedData>>(std::unique_ptr<TypedData>(TypedData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<TypedData>, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline const diplomat::capi::TypedData* TypedData::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::TypedData*>(this);
}

inline diplomat::capi::TypedData* TypedData::AsFFI() {
  return reinterpret_cast<diplomat::capi::TypedData*>(this);
}

inline const TypedData* TypedData::FromFFI(const diplomat::capi::TypedData* ptr) {
  return reinterpret_cast<const TypedData*>(ptr);
}

inline TypedData* TypedData::FromFFI(diplomat::capi::TypedData* ptr) {
  return reinterpret_cast<TypedData*>(ptr);
}

inline void TypedData::operator delete(void* ptr) {
  diplomat::capi::TypedData_destroy(reinterpret_cast<diplomat::capi::TypedData*>(ptr));
}


#endif // TypedData_HPP
