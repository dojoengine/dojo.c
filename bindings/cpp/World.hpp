#ifndef World_HPP
#define World_HPP

#include "World.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "DojoError.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {

    void World_world_address(const diplomat::capi::World* self, diplomat::capi::DiplomatWrite* write);

    uint32_t World_models_count(const diplomat::capi::World* self);

    typedef struct World_from_json_result {union {diplomat::capi::World* ok; diplomat::capi::DojoError* err;}; bool is_ok;} World_from_json_result;
    World_from_json_result World_from_json(diplomat::capi::DiplomatStringView json);

    typedef struct World_to_json_result {union { diplomat::capi::DojoError* err;}; bool is_ok;} World_to_json_result;
    World_to_json_result World_to_json(const diplomat::capi::World* self, diplomat::capi::DiplomatWrite* write);

    void World_destroy(World* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::string World::world_address() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::World_world_address(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void World::world_address_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::World_world_address(this->AsFFI(),
    &write);
}

inline uint32_t World::models_count() const {
  auto result = diplomat::capi::World_models_count(this->AsFFI());
  return result;
}

inline diplomat::result<std::unique_ptr<World>, std::unique_ptr<DojoError>> World::from_json(std::string_view json) {
  auto result = diplomat::capi::World_from_json({json.data(), json.size()});
  return result.is_ok ? diplomat::result<std::unique_ptr<World>, std::unique_ptr<DojoError>>(diplomat::Ok<std::unique_ptr<World>>(std::unique_ptr<World>(World::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<World>, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<DojoError>> World::to_json() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::World_to_json(this->AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, std::unique_ptr<DojoError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> World::to_json_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  auto result = diplomat::capi::World_to_json(this->AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline const diplomat::capi::World* World::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::World*>(this);
}

inline diplomat::capi::World* World::AsFFI() {
  return reinterpret_cast<diplomat::capi::World*>(this);
}

inline const World* World::FromFFI(const diplomat::capi::World* ptr) {
  return reinterpret_cast<const World*>(ptr);
}

inline World* World::FromFFI(diplomat::capi::World* ptr) {
  return reinterpret_cast<World*>(ptr);
}

inline void World::operator delete(void* ptr) {
  diplomat::capi::World_destroy(reinterpret_cast<diplomat::capi::World*>(ptr));
}


#endif // World_HPP
