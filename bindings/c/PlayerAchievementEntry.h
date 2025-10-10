#ifndef PlayerAchievementEntry_H
#define PlayerAchievementEntry_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DojoError.d.h"

#include "PlayerAchievementEntry.d.h"






void PlayerAchievementEntry_player_address(const PlayerAchievementEntry* self, DiplomatWrite* write);

uint32_t PlayerAchievementEntry_total_points(const PlayerAchievementEntry* self);

uint32_t PlayerAchievementEntry_completed_achievements(const PlayerAchievementEntry* self);

uint32_t PlayerAchievementEntry_total_achievements(const PlayerAchievementEntry* self);

double PlayerAchievementEntry_completion_percentage(const PlayerAchievementEntry* self);

uint32_t PlayerAchievementEntry_achievements_count(const PlayerAchievementEntry* self);

uint64_t PlayerAchievementEntry_updated_at(const PlayerAchievementEntry* self);

typedef struct PlayerAchievementEntry_from_json_result {union {PlayerAchievementEntry* ok; DojoError* err;}; bool is_ok;} PlayerAchievementEntry_from_json_result;
PlayerAchievementEntry_from_json_result PlayerAchievementEntry_from_json(DiplomatStringView json);

typedef struct PlayerAchievementEntry_to_json_result {union { DojoError* err;}; bool is_ok;} PlayerAchievementEntry_to_json_result;
PlayerAchievementEntry_to_json_result PlayerAchievementEntry_to_json(const PlayerAchievementEntry* self, DiplomatWrite* write);

void PlayerAchievementEntry_destroy(PlayerAchievementEntry* self);





#endif // PlayerAchievementEntry_H
