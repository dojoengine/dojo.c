#ifndef Call_HPP
#define Call_HPP

#include "Call.d.hpp"

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

    diplomat::capi::Call* Call_new(const diplomat::capi::FieldElement* to, const diplomat::capi::FieldElement* selector);

    void Call_push_calldata(diplomat::capi::Call* self, const diplomat::capi::FieldElement* felt);

    typedef struct Call_new_from_selector_name_result {union {diplomat::capi::Call* ok; diplomat::capi::DojoError* err;}; bool is_ok;} Call_new_from_selector_name_result;
    Call_new_from_selector_name_result Call_new_from_selector_name(const diplomat::capi::FieldElement* to, diplomat::capi::DiplomatStringView selector_name);

    void Call_destroy(Call* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<Call> Call::new_(const FieldElement& to, const FieldElement& selector) {
  auto result = diplomat::capi::Call_new(to.AsFFI(),
    selector.AsFFI());
  return std::unique_ptr<Call>(Call::FromFFI(result));
}

inline void Call::push_calldata(const FieldElement& felt) {
  diplomat::capi::Call_push_calldata(this->AsFFI(),
    felt.AsFFI());
}

inline diplomat::result<std::unique_ptr<Call>, std::unique_ptr<DojoError>> Call::new_from_selector_name(const FieldElement& to, std::string_view selector_name) {
  auto result = diplomat::capi::Call_new_from_selector_name(to.AsFFI(),
    {selector_name.data(), selector_name.size()});
  return result.is_ok ? diplomat::result<std::unique_ptr<Call>, std::unique_ptr<DojoError>>(diplomat::Ok<std::unique_ptr<Call>>(std::unique_ptr<Call>(Call::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<Call>, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline const diplomat::capi::Call* Call::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::Call*>(this);
}

inline diplomat::capi::Call* Call::AsFFI() {
  return reinterpret_cast<diplomat::capi::Call*>(this);
}

inline const Call* Call::FromFFI(const diplomat::capi::Call* ptr) {
  return reinterpret_cast<const Call*>(ptr);
}

inline Call* Call::FromFFI(diplomat::capi::Call* ptr) {
  return reinterpret_cast<Call*>(ptr);
}

inline void Call::operator delete(void* ptr) {
  diplomat::capi::Call_destroy(reinterpret_cast<diplomat::capi::Call*>(ptr));
}


#endif // Call_HPP
