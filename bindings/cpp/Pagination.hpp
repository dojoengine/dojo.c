#ifndef Pagination_HPP
#define Pagination_HPP

#include "Pagination.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "DojoError.hpp"
#include "OrderBy.hpp"
#include "PaginationDirection.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {

    diplomat::capi::Pagination* Pagination_new(void);

    typedef struct Pagination_set_cursor_result {union { diplomat::capi::DojoError* err;}; bool is_ok;} Pagination_set_cursor_result;
    Pagination_set_cursor_result Pagination_set_cursor(diplomat::capi::Pagination* self, diplomat::capi::DiplomatStringView cursor);

    void Pagination_set_limit(diplomat::capi::Pagination* self, uint32_t limit);

    void Pagination_set_direction(diplomat::capi::Pagination* self, diplomat::capi::PaginationDirection direction);

    void Pagination_add_order_by(diplomat::capi::Pagination* self, const diplomat::capi::OrderBy* order_by);

    void Pagination_destroy(Pagination* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<Pagination> Pagination::new_() {
  auto result = diplomat::capi::Pagination_new();
  return std::unique_ptr<Pagination>(Pagination::FromFFI(result));
}

inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> Pagination::set_cursor(std::string_view cursor) {
  auto result = diplomat::capi::Pagination_set_cursor(this->AsFFI(),
    {cursor.data(), cursor.size()});
  return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline void Pagination::set_limit(uint32_t limit) {
  diplomat::capi::Pagination_set_limit(this->AsFFI(),
    limit);
}

inline void Pagination::set_direction(PaginationDirection direction) {
  diplomat::capi::Pagination_set_direction(this->AsFFI(),
    direction.AsFFI());
}

inline void Pagination::add_order_by(const OrderBy& order_by) {
  diplomat::capi::Pagination_add_order_by(this->AsFFI(),
    order_by.AsFFI());
}

inline const diplomat::capi::Pagination* Pagination::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::Pagination*>(this);
}

inline diplomat::capi::Pagination* Pagination::AsFFI() {
  return reinterpret_cast<diplomat::capi::Pagination*>(this);
}

inline const Pagination* Pagination::FromFFI(const diplomat::capi::Pagination* ptr) {
  return reinterpret_cast<const Pagination*>(ptr);
}

inline Pagination* Pagination::FromFFI(diplomat::capi::Pagination* ptr) {
  return reinterpret_cast<Pagination*>(ptr);
}

inline void Pagination::operator delete(void* ptr) {
  diplomat::capi::Pagination_destroy(reinterpret_cast<diplomat::capi::Pagination*>(ptr));
}


#endif // Pagination_HPP
