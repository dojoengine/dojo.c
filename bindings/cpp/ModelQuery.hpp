#ifndef ModelQuery_HPP
#define ModelQuery_HPP

#include "ModelQuery.d.hpp"

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

    diplomat::capi::ModelQuery* ModelQuery_new(void);

    typedef struct ModelQuery_add_model_result {union { diplomat::capi::DojoError* err;}; bool is_ok;} ModelQuery_add_model_result;
    ModelQuery_add_model_result ModelQuery_add_model(diplomat::capi::ModelQuery* self, diplomat::capi::DiplomatStringView model_name);

    void ModelQuery_destroy(ModelQuery* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<ModelQuery> ModelQuery::new_() {
  auto result = diplomat::capi::ModelQuery_new();
  return std::unique_ptr<ModelQuery>(ModelQuery::FromFFI(result));
}

inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> ModelQuery::add_model(std::string_view model_name) {
  auto result = diplomat::capi::ModelQuery_add_model(this->AsFFI(),
    {model_name.data(), model_name.size()});
  return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline const diplomat::capi::ModelQuery* ModelQuery::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::ModelQuery*>(this);
}

inline diplomat::capi::ModelQuery* ModelQuery::AsFFI() {
  return reinterpret_cast<diplomat::capi::ModelQuery*>(this);
}

inline const ModelQuery* ModelQuery::FromFFI(const diplomat::capi::ModelQuery* ptr) {
  return reinterpret_cast<const ModelQuery*>(ptr);
}

inline ModelQuery* ModelQuery::FromFFI(diplomat::capi::ModelQuery* ptr) {
  return reinterpret_cast<ModelQuery*>(ptr);
}

inline void ModelQuery::operator delete(void* ptr) {
  diplomat::capi::ModelQuery_destroy(reinterpret_cast<diplomat::capi::ModelQuery*>(ptr));
}


#endif // ModelQuery_HPP
