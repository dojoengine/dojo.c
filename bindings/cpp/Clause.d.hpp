#ifndef Clause_D_HPP
#define Clause_D_HPP

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
    struct Clause;
} // namespace capi
} // namespace

/**
 * Query clause for filtering entities
 */
class Clause {
public:

  inline const diplomat::capi::Clause* AsFFI() const;
  inline diplomat::capi::Clause* AsFFI();
  inline static const Clause* FromFFI(const diplomat::capi::Clause* ptr);
  inline static Clause* FromFFI(diplomat::capi::Clause* ptr);
  inline static void operator delete(void* ptr);
private:
  Clause() = delete;
  Clause(const Clause&) = delete;
  Clause(Clause&&) noexcept = delete;
  Clause operator=(const Clause&) = delete;
  Clause operator=(Clause&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // Clause_D_HPP
