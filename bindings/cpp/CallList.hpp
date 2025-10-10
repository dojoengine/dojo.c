#ifndef CallList_HPP
#define CallList_HPP

#include "CallList.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "Call.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {

    diplomat::capi::CallList* CallList_new(void);

    void CallList_add_call(diplomat::capi::CallList* self, const diplomat::capi::Call* call);

    size_t CallList_len(const diplomat::capi::CallList* self);

    void CallList_destroy(CallList* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<CallList> CallList::new_() {
  auto result = diplomat::capi::CallList_new();
  return std::unique_ptr<CallList>(CallList::FromFFI(result));
}

inline void CallList::add_call(const Call& call) {
  diplomat::capi::CallList_add_call(this->AsFFI(),
    call.AsFFI());
}

inline size_t CallList::len() const {
  auto result = diplomat::capi::CallList_len(this->AsFFI());
  return result;
}

inline const diplomat::capi::CallList* CallList::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::CallList*>(this);
}

inline diplomat::capi::CallList* CallList::AsFFI() {
  return reinterpret_cast<diplomat::capi::CallList*>(this);
}

inline const CallList* CallList::FromFFI(const diplomat::capi::CallList* ptr) {
  return reinterpret_cast<const CallList*>(ptr);
}

inline CallList* CallList::FromFFI(diplomat::capi::CallList* ptr) {
  return reinterpret_cast<CallList*>(ptr);
}

inline void CallList::operator delete(void* ptr) {
  diplomat::capi::CallList_destroy(reinterpret_cast<diplomat::capi::CallList*>(ptr));
}


#endif // CallList_HPP
