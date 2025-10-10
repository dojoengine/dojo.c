#ifndef EntityQuery_HPP
#define EntityQuery_HPP

#include "EntityQuery.d.hpp"

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
    extern "C" {

    diplomat::capi::EntityQuery* EntityQuery_new(void);

    void EntityQuery_set_limit(diplomat::capi::EntityQuery* self, uint32_t limit);

    void EntityQuery_set_offset(diplomat::capi::EntityQuery* self, uint32_t offset);

    void EntityQuery_destroy(EntityQuery* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<EntityQuery> EntityQuery::new_() {
  auto result = diplomat::capi::EntityQuery_new();
  return std::unique_ptr<EntityQuery>(EntityQuery::FromFFI(result));
}

inline void EntityQuery::set_limit(uint32_t limit) {
  diplomat::capi::EntityQuery_set_limit(this->AsFFI(),
    limit);
}

inline void EntityQuery::set_offset(uint32_t offset) {
  diplomat::capi::EntityQuery_set_offset(this->AsFFI(),
    offset);
}

inline const diplomat::capi::EntityQuery* EntityQuery::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::EntityQuery*>(this);
}

inline diplomat::capi::EntityQuery* EntityQuery::AsFFI() {
  return reinterpret_cast<diplomat::capi::EntityQuery*>(this);
}

inline const EntityQuery* EntityQuery::FromFFI(const diplomat::capi::EntityQuery* ptr) {
  return reinterpret_cast<const EntityQuery*>(ptr);
}

inline EntityQuery* EntityQuery::FromFFI(diplomat::capi::EntityQuery* ptr) {
  return reinterpret_cast<EntityQuery*>(ptr);
}

inline void EntityQuery::operator delete(void* ptr) {
  diplomat::capi::EntityQuery_destroy(reinterpret_cast<diplomat::capi::EntityQuery*>(ptr));
}


#endif // EntityQuery_HPP
