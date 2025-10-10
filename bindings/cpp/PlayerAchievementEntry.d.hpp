#ifndef PlayerAchievementEntry_D_HPP
#define PlayerAchievementEntry_D_HPP

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
    struct PlayerAchievementEntry;
} // namespace capi
} // namespace

/**
 * Represents player achievement progress
 */
class PlayerAchievementEntry {
public:

  /**
   * Gets the player address (hex)
   */
  inline std::string player_address() const;
  template<typename W>
  inline void player_address_write(W& writeable_output) const;

  /**
   * Gets the total points
   */
  inline uint32_t total_points() const;

  /**
   * Gets the completed achievements count
   */
  inline uint32_t completed_achievements() const;

  /**
   * Gets the total achievements count
   */
  inline uint32_t total_achievements() const;

  /**
   * Gets the completion percentage
   */
  inline double completion_percentage() const;

  /**
   * Gets the achievements count
   */
  inline uint32_t achievements_count() const;

  /**
   * Gets the updated_at timestamp
   */
  inline uint64_t updated_at() const;

  /**
   * Creates a player achievement entry from JSON
   */
  inline static diplomat::result<std::unique_ptr<PlayerAchievementEntry>, std::unique_ptr<DojoError>> from_json(std::string_view json);

  /**
   * Serializes the player achievement entry to JSON
   */
  inline diplomat::result<std::string, std::unique_ptr<DojoError>> to_json() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> to_json_write(W& writeable_output) const;

  inline const diplomat::capi::PlayerAchievementEntry* AsFFI() const;
  inline diplomat::capi::PlayerAchievementEntry* AsFFI();
  inline static const PlayerAchievementEntry* FromFFI(const diplomat::capi::PlayerAchievementEntry* ptr);
  inline static PlayerAchievementEntry* FromFFI(diplomat::capi::PlayerAchievementEntry* ptr);
  inline static void operator delete(void* ptr);
private:
  PlayerAchievementEntry() = delete;
  PlayerAchievementEntry(const PlayerAchievementEntry&) = delete;
  PlayerAchievementEntry(PlayerAchievementEntry&&) noexcept = delete;
  PlayerAchievementEntry operator=(const PlayerAchievementEntry&) = delete;
  PlayerAchievementEntry operator=(PlayerAchievementEntry&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // PlayerAchievementEntry_D_HPP
