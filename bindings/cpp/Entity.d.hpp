#ifndef Entity_D_HPP
#define Entity_D_HPP

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


namespace diplomat {
namespace capi {
    struct Entity;
} // namespace capi
} // namespace

/**
 * Represents a Dojo entity
 */
class Entity {
public:

  /**
   * Gets the hashed keys (hex)
   */
  inline std::string hashed_keys() const;
  template<typename W>
  inline void hashed_keys_write(W& writeable_output) const;

  /**
   * Gets the number of models in this entity
   */
  inline uint32_t models_count() const;

  /**
   * Gets the created_at timestamp
   */
  inline uint64_t created_at() const;

  /**
   * Gets the updated_at timestamp
   */
  inline uint64_t updated_at() const;

  /**
   * Gets the executed_at timestamp
   */
  inline uint64_t executed_at() const;

  /**
   * Creates a new entity from JSON
   */
  inline static diplomat::result<std::unique_ptr<Entity>, std::unique_ptr<DojoError>> from_json(std::string_view json);

  /**
   * Serializes the entity to JSON
   */
  inline diplomat::result<std::string, std::unique_ptr<DojoError>> to_json() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> to_json_write(W& writeable_output) const;

  inline const diplomat::capi::Entity* AsFFI() const;
  inline diplomat::capi::Entity* AsFFI();
  inline static const Entity* FromFFI(const diplomat::capi::Entity* ptr);
  inline static Entity* FromFFI(diplomat::capi::Entity* ptr);
  inline static void operator delete(void* ptr);
private:
  Entity() = delete;
  Entity(const Entity&) = delete;
  Entity(Entity&&) noexcept = delete;
  Entity operator=(const Entity&) = delete;
  Entity operator=(Entity&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // Entity_D_HPP
