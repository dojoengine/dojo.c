#ifndef World_D_HPP
#define World_D_HPP

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
    struct World;
} // namespace capi
} // namespace

/**
 * Represents a Dojo world
 */
class World {
public:

  /**
   * Gets the world address (hex)
   */
  inline std::string world_address() const;
  template<typename W>
  inline void world_address_write(W& writeable_output) const;

  /**
   * Gets the number of models in this world
   */
  inline uint32_t models_count() const;

  /**
   * Creates a new world from JSON
   */
  inline static diplomat::result<std::unique_ptr<World>, std::unique_ptr<DojoError>> from_json(std::string_view json);

  /**
   * Serializes the world to JSON
   */
  inline diplomat::result<std::string, std::unique_ptr<DojoError>> to_json() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> to_json_write(W& writeable_output) const;

  inline const diplomat::capi::World* AsFFI() const;
  inline diplomat::capi::World* AsFFI();
  inline static const World* FromFFI(const diplomat::capi::World* ptr);
  inline static World* FromFFI(diplomat::capi::World* ptr);
  inline static void operator delete(void* ptr);
private:
  World() = delete;
  World(const World&) = delete;
  World(World&&) noexcept = delete;
  World operator=(const World&) = delete;
  World operator=(World&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // World_D_HPP
