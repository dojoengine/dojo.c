#ifndef PlayerAchievementEntry_HPP
#define PlayerAchievementEntry_HPP

#include "PlayerAchievementEntry.d.hpp"

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

    void PlayerAchievementEntry_player_address(const diplomat::capi::PlayerAchievementEntry* self, diplomat::capi::DiplomatWrite* write);

    uint32_t PlayerAchievementEntry_total_points(const diplomat::capi::PlayerAchievementEntry* self);

    uint32_t PlayerAchievementEntry_completed_achievements(const diplomat::capi::PlayerAchievementEntry* self);

    uint32_t PlayerAchievementEntry_total_achievements(const diplomat::capi::PlayerAchievementEntry* self);

    double PlayerAchievementEntry_completion_percentage(const diplomat::capi::PlayerAchievementEntry* self);

    uint32_t PlayerAchievementEntry_achievements_count(const diplomat::capi::PlayerAchievementEntry* self);

    uint64_t PlayerAchievementEntry_updated_at(const diplomat::capi::PlayerAchievementEntry* self);

    typedef struct PlayerAchievementEntry_from_json_result {union {diplomat::capi::PlayerAchievementEntry* ok; diplomat::capi::DojoError* err;}; bool is_ok;} PlayerAchievementEntry_from_json_result;
    PlayerAchievementEntry_from_json_result PlayerAchievementEntry_from_json(diplomat::capi::DiplomatStringView json);

    typedef struct PlayerAchievementEntry_to_json_result {union { diplomat::capi::DojoError* err;}; bool is_ok;} PlayerAchievementEntry_to_json_result;
    PlayerAchievementEntry_to_json_result PlayerAchievementEntry_to_json(const diplomat::capi::PlayerAchievementEntry* self, diplomat::capi::DiplomatWrite* write);

    void PlayerAchievementEntry_destroy(PlayerAchievementEntry* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::string PlayerAchievementEntry::player_address() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::PlayerAchievementEntry_player_address(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void PlayerAchievementEntry::player_address_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::PlayerAchievementEntry_player_address(this->AsFFI(),
    &write);
}

inline uint32_t PlayerAchievementEntry::total_points() const {
  auto result = diplomat::capi::PlayerAchievementEntry_total_points(this->AsFFI());
  return result;
}

inline uint32_t PlayerAchievementEntry::completed_achievements() const {
  auto result = diplomat::capi::PlayerAchievementEntry_completed_achievements(this->AsFFI());
  return result;
}

inline uint32_t PlayerAchievementEntry::total_achievements() const {
  auto result = diplomat::capi::PlayerAchievementEntry_total_achievements(this->AsFFI());
  return result;
}

inline double PlayerAchievementEntry::completion_percentage() const {
  auto result = diplomat::capi::PlayerAchievementEntry_completion_percentage(this->AsFFI());
  return result;
}

inline uint32_t PlayerAchievementEntry::achievements_count() const {
  auto result = diplomat::capi::PlayerAchievementEntry_achievements_count(this->AsFFI());
  return result;
}

inline uint64_t PlayerAchievementEntry::updated_at() const {
  auto result = diplomat::capi::PlayerAchievementEntry_updated_at(this->AsFFI());
  return result;
}

inline diplomat::result<std::unique_ptr<PlayerAchievementEntry>, std::unique_ptr<DojoError>> PlayerAchievementEntry::from_json(std::string_view json) {
  auto result = diplomat::capi::PlayerAchievementEntry_from_json({json.data(), json.size()});
  return result.is_ok ? diplomat::result<std::unique_ptr<PlayerAchievementEntry>, std::unique_ptr<DojoError>>(diplomat::Ok<std::unique_ptr<PlayerAchievementEntry>>(std::unique_ptr<PlayerAchievementEntry>(PlayerAchievementEntry::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<PlayerAchievementEntry>, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<DojoError>> PlayerAchievementEntry::to_json() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::PlayerAchievementEntry_to_json(this->AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, std::unique_ptr<DojoError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> PlayerAchievementEntry::to_json_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  auto result = diplomat::capi::PlayerAchievementEntry_to_json(this->AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline const diplomat::capi::PlayerAchievementEntry* PlayerAchievementEntry::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::PlayerAchievementEntry*>(this);
}

inline diplomat::capi::PlayerAchievementEntry* PlayerAchievementEntry::AsFFI() {
  return reinterpret_cast<diplomat::capi::PlayerAchievementEntry*>(this);
}

inline const PlayerAchievementEntry* PlayerAchievementEntry::FromFFI(const diplomat::capi::PlayerAchievementEntry* ptr) {
  return reinterpret_cast<const PlayerAchievementEntry*>(ptr);
}

inline PlayerAchievementEntry* PlayerAchievementEntry::FromFFI(diplomat::capi::PlayerAchievementEntry* ptr) {
  return reinterpret_cast<PlayerAchievementEntry*>(ptr);
}

inline void PlayerAchievementEntry::operator delete(void* ptr) {
  diplomat::capi::PlayerAchievementEntry_destroy(reinterpret_cast<diplomat::capi::PlayerAchievementEntry*>(ptr));
}


#endif // PlayerAchievementEntry_HPP
