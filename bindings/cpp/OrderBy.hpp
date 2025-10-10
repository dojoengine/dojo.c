#ifndef OrderBy_HPP
#define OrderBy_HPP

#include "OrderBy.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "DojoError.hpp"
#include "OrderDirection.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {

    typedef struct OrderBy_new_result {union {diplomat::capi::OrderBy* ok; diplomat::capi::DojoError* err;}; bool is_ok;} OrderBy_new_result;
    OrderBy_new_result OrderBy_new(diplomat::capi::DiplomatStringView field, diplomat::capi::OrderDirection direction);

    void OrderBy_destroy(OrderBy* self);

    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<OrderBy>, std::unique_ptr<DojoError>> OrderBy::new_(std::string_view field, OrderDirection direction) {
  auto result = diplomat::capi::OrderBy_new({field.data(), field.size()},
    direction.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<OrderBy>, std::unique_ptr<DojoError>>(diplomat::Ok<std::unique_ptr<OrderBy>>(std::unique_ptr<OrderBy>(OrderBy::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<OrderBy>, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline const diplomat::capi::OrderBy* OrderBy::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::OrderBy*>(this);
}

inline diplomat::capi::OrderBy* OrderBy::AsFFI() {
  return reinterpret_cast<diplomat::capi::OrderBy*>(this);
}

inline const OrderBy* OrderBy::FromFFI(const diplomat::capi::OrderBy* ptr) {
  return reinterpret_cast<const OrderBy*>(ptr);
}

inline OrderBy* OrderBy::FromFFI(diplomat::capi::OrderBy* ptr) {
  return reinterpret_cast<OrderBy*>(ptr);
}

inline void OrderBy::operator delete(void* ptr) {
  diplomat::capi::OrderBy_destroy(reinterpret_cast<diplomat::capi::OrderBy*>(ptr));
}


#endif // OrderBy_HPP
