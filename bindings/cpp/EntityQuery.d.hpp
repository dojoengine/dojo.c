#ifndef EntityQuery_D_HPP
#define EntityQuery_D_HPP

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
    struct EntityQuery;
} // namespace capi
} // namespace

/**
 * Entity query for retrieving entities from Torii
 */
class EntityQuery {
public:

  /**
   * Creates a new entity query
   */
  inline static std::unique_ptr<EntityQuery> new_();

  /**
   * Sets the limit for the query
   */
  inline void set_limit(uint32_t limit);

  /**
   * Sets the offset for the query
   */
  inline void set_offset(uint32_t offset);

  inline const diplomat::capi::EntityQuery* AsFFI() const;
  inline diplomat::capi::EntityQuery* AsFFI();
  inline static const EntityQuery* FromFFI(const diplomat::capi::EntityQuery* ptr);
  inline static EntityQuery* FromFFI(diplomat::capi::EntityQuery* ptr);
  inline static void operator delete(void* ptr);
private:
  EntityQuery() = delete;
  EntityQuery(const EntityQuery&) = delete;
  EntityQuery(EntityQuery&&) noexcept = delete;
  EntityQuery operator=(const EntityQuery&) = delete;
  EntityQuery operator=(EntityQuery&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // EntityQuery_D_HPP
