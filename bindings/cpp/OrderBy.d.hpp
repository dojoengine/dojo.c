#ifndef OrderBy_D_HPP
#define OrderBy_D_HPP

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
class OrderDirection;


namespace diplomat {
namespace capi {
    struct OrderBy;
} // namespace capi
} // namespace

/**
 * Ordering specification for query results
 */
class OrderBy {
public:

  /**
   * Creates a new OrderBy specification
   */
  inline static diplomat::result<std::unique_ptr<OrderBy>, std::unique_ptr<DojoError>> new_(std::string_view field, OrderDirection direction);

  inline const diplomat::capi::OrderBy* AsFFI() const;
  inline diplomat::capi::OrderBy* AsFFI();
  inline static const OrderBy* FromFFI(const diplomat::capi::OrderBy* ptr);
  inline static OrderBy* FromFFI(diplomat::capi::OrderBy* ptr);
  inline static void operator delete(void* ptr);
private:
  OrderBy() = delete;
  OrderBy(const OrderBy&) = delete;
  OrderBy(OrderBy&&) noexcept = delete;
  OrderBy operator=(const OrderBy&) = delete;
  OrderBy operator=(OrderBy&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // OrderBy_D_HPP
