#ifndef KeysClause_HPP
#define KeysClause_HPP

#include "KeysClause.d.hpp"

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

    diplomat::capi::KeysClause* KeysClause_new(void);

    void KeysClause_add_key(diplomat::capi::KeysClause* self, const diplomat::capi::FieldElement* key);

    typedef struct KeysClause_add_model_result {union { diplomat::capi::DojoError* err;}; bool is_ok;} KeysClause_add_model_result;
    KeysClause_add_model_result KeysClause_add_model(diplomat::capi::KeysClause* self, diplomat::capi::DiplomatStringView model);

    void KeysClause_destroy(KeysClause* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<KeysClause> KeysClause::new_() {
  auto result = diplomat::capi::KeysClause_new();
  return std::unique_ptr<KeysClause>(KeysClause::FromFFI(result));
}

inline void KeysClause::add_key(const FieldElement& key) {
  diplomat::capi::KeysClause_add_key(this->AsFFI(),
    key.AsFFI());
}

inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> KeysClause::add_model(std::string_view model) {
  auto result = diplomat::capi::KeysClause_add_model(this->AsFFI(),
    {model.data(), model.size()});
  return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline const diplomat::capi::KeysClause* KeysClause::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::KeysClause*>(this);
}

inline diplomat::capi::KeysClause* KeysClause::AsFFI() {
  return reinterpret_cast<diplomat::capi::KeysClause*>(this);
}

inline const KeysClause* KeysClause::FromFFI(const diplomat::capi::KeysClause* ptr) {
  return reinterpret_cast<const KeysClause*>(ptr);
}

inline KeysClause* KeysClause::FromFFI(diplomat::capi::KeysClause* ptr) {
  return reinterpret_cast<KeysClause*>(ptr);
}

inline void KeysClause::operator delete(void* ptr) {
  diplomat::capi::KeysClause_destroy(reinterpret_cast<diplomat::capi::KeysClause*>(ptr));
}


#endif // KeysClause_HPP
