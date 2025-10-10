#ifndef Pagination_D_HPP
#define Pagination_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"

namespace diplomat::capi { struct DojoError; }
class DojoError;
namespace diplomat::capi { struct OrderBy; }
class OrderBy;
class PaginationDirection;


namespace diplomat {
namespace capi {
    struct Pagination;
} // namespace capi
} // namespace

/**
 * Pagination configuration for queries
 */
class Pagination {
public:

  /**
   * Creates a new Pagination with default values
   */
  inline static std::unique_ptr<Pagination> new_();

  /**
   * Sets the cursor for pagination
   */
  inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> set_cursor(std::string_view cursor);

  /**
   * Sets the limit for pagination
   */
  inline void set_limit(uint32_t limit);

  /**
   * Sets the direction for pagination
   */
  inline void set_direction(PaginationDirection direction);

  /**
   * Adds an ordering specification
   */
  inline void add_order_by(const OrderBy& order_by);

  inline const diplomat::capi::Pagination* AsFFI() const;
  inline diplomat::capi::Pagination* AsFFI();
  inline static const Pagination* FromFFI(const diplomat::capi::Pagination* ptr);
  inline static Pagination* FromFFI(diplomat::capi::Pagination* ptr);
  inline static void operator delete(void* ptr);
private:
  Pagination() = delete;
  Pagination(const Pagination&) = delete;
  Pagination(Pagination&&) noexcept = delete;
  Pagination operator=(const Pagination&) = delete;
  Pagination operator=(Pagination&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // Pagination_D_HPP
