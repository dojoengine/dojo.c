#ifndef Entity_HPP
#define Entity_HPP

#include "Entity.d.hpp"

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

    void Entity_hashed_keys(const diplomat::capi::Entity* self, diplomat::capi::DiplomatWrite* write);

    uint32_t Entity_models_count(const diplomat::capi::Entity* self);

    uint64_t Entity_created_at(const diplomat::capi::Entity* self);

    uint64_t Entity_updated_at(const diplomat::capi::Entity* self);

    uint64_t Entity_executed_at(const diplomat::capi::Entity* self);

    typedef struct Entity_from_json_result {union {diplomat::capi::Entity* ok; diplomat::capi::DojoError* err;}; bool is_ok;} Entity_from_json_result;
    Entity_from_json_result Entity_from_json(diplomat::capi::DiplomatStringView json);

    typedef struct Entity_to_json_result {union { diplomat::capi::DojoError* err;}; bool is_ok;} Entity_to_json_result;
    Entity_to_json_result Entity_to_json(const diplomat::capi::Entity* self, diplomat::capi::DiplomatWrite* write);

    void Entity_destroy(Entity* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::string Entity::hashed_keys() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::Entity_hashed_keys(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void Entity::hashed_keys_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::Entity_hashed_keys(this->AsFFI(),
    &write);
}

inline uint32_t Entity::models_count() const {
  auto result = diplomat::capi::Entity_models_count(this->AsFFI());
  return result;
}

inline uint64_t Entity::created_at() const {
  auto result = diplomat::capi::Entity_created_at(this->AsFFI());
  return result;
}

inline uint64_t Entity::updated_at() const {
  auto result = diplomat::capi::Entity_updated_at(this->AsFFI());
  return result;
}

inline uint64_t Entity::executed_at() const {
  auto result = diplomat::capi::Entity_executed_at(this->AsFFI());
  return result;
}

inline diplomat::result<std::unique_ptr<Entity>, std::unique_ptr<DojoError>> Entity::from_json(std::string_view json) {
  auto result = diplomat::capi::Entity_from_json({json.data(), json.size()});
  return result.is_ok ? diplomat::result<std::unique_ptr<Entity>, std::unique_ptr<DojoError>>(diplomat::Ok<std::unique_ptr<Entity>>(std::unique_ptr<Entity>(Entity::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<Entity>, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<DojoError>> Entity::to_json() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::Entity_to_json(this->AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, std::unique_ptr<DojoError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> Entity::to_json_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  auto result = diplomat::capi::Entity_to_json(this->AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline const diplomat::capi::Entity* Entity::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::Entity*>(this);
}

inline diplomat::capi::Entity* Entity::AsFFI() {
  return reinterpret_cast<diplomat::capi::Entity*>(this);
}

inline const Entity* Entity::FromFFI(const diplomat::capi::Entity* ptr) {
  return reinterpret_cast<const Entity*>(ptr);
}

inline Entity* Entity::FromFFI(diplomat::capi::Entity* ptr) {
  return reinterpret_cast<Entity*>(ptr);
}

inline void Entity::operator delete(void* ptr) {
  diplomat::capi::Entity_destroy(reinterpret_cast<diplomat::capi::Entity*>(ptr));
}


#endif // Entity_HPP
