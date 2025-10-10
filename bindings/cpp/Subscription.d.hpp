#ifndef Subscription_D_HPP
#define Subscription_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    struct Subscription;
} // namespace capi
} // namespace

/**
 * Subscription handle for managing event streams
 */
class Subscription {
public:

  /**
   * Gets the subscription ID
   */
  inline uint64_t id() const;

  inline const diplomat::capi::Subscription* AsFFI() const;
  inline diplomat::capi::Subscription* AsFFI();
  inline static const Subscription* FromFFI(const diplomat::capi::Subscription* ptr);
  inline static Subscription* FromFFI(diplomat::capi::Subscription* ptr);
  inline static void operator delete(void* ptr);
private:
  Subscription() = delete;
  Subscription(const Subscription&) = delete;
  Subscription(Subscription&&) noexcept = delete;
  Subscription operator=(const Subscription&) = delete;
  Subscription operator=(Subscription&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // Subscription_D_HPP
