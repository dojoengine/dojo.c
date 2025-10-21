// Achievement types
use super::core::*;

#[derive(Debug, Clone)]
pub struct AchievementTask {
    pub task_id: String,
    pub description: String,
    pub total: u32,
    pub total_completions: u32,
    pub completion_rate: f64,
    pub created_at: u64,
}

impl From<torii_proto::AchievementTask> for AchievementTask {
    fn from(val: torii_proto::AchievementTask) -> Self {
        AchievementTask {
            task_id: val.task_id,
            description: val.description,
            total: val.total,
            total_completions: val.total_completions,
            completion_rate: val.completion_rate,
            created_at: val.created_at.timestamp() as u64,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Achievement {
    pub id: String,
    pub world_address: FieldElement,
    pub namespace: String,
    pub entity_id: String,
    pub hidden: bool,
    pub index: u32,
    pub points: u32,
    pub start: String,
    pub end: String,
    pub group: String,
    pub icon: String,
    pub title: String,
    pub description: String,
    pub tasks: Vec<AchievementTask>,
    pub data: Option<String>,
    pub total_completions: u32,
    pub completion_rate: f64,
    pub created_at: u64,
    pub updated_at: u64,
}

impl From<torii_proto::Achievement> for Achievement {
    fn from(val: torii_proto::Achievement) -> Self {
        let tasks: Vec<AchievementTask> = val.tasks.into_iter().map(|t| t.into()).collect();

        Achievement {
            id: val.id,
            world_address: felt_to_field_element(val.world_address),
            namespace: val.namespace,
            entity_id: val.entity_id,
            hidden: val.hidden,
            index: val.index,
            points: val.points,
            start: val.start,
            end: val.end,
            group: val.group,
            icon: val.icon,
            title: val.title,
            description: val.description,
            tasks,
            data: val.data,
            total_completions: val.total_completions,
            completion_rate: val.completion_rate,
            created_at: val.created_at.timestamp() as u64,
            updated_at: val.updated_at.timestamp() as u64,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AchievementQuery {
    pub world_addresses: Vec<FieldElement>,
    pub namespaces: Vec<String>,
    pub hidden: Option<bool>,
    pub pagination: Pagination,
}

impl From<AchievementQuery> for torii_proto::AchievementQuery {
    fn from(val: AchievementQuery) -> Self {
        torii_proto::AchievementQuery {
            world_addresses: val
                .world_addresses
                .into_iter()
                .map(|a| field_element_to_felt(&a).unwrap())
                .collect(),
            namespaces: val.namespaces,
            hidden: val.hidden,
            pagination: val.pagination.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TaskProgress {
    pub task_id: String,
    pub count: u32,
    pub completed: bool,
}

impl From<torii_proto::TaskProgress> for TaskProgress {
    fn from(val: torii_proto::TaskProgress) -> Self {
        TaskProgress { task_id: val.task_id, count: val.count, completed: val.completed }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerAchievementProgress {
    pub achievement: Achievement,
    pub task_progress: Vec<TaskProgress>,
    pub completed: bool,
    pub progress_percentage: f64,
}

impl From<torii_proto::PlayerAchievementProgress> for PlayerAchievementProgress {
    fn from(val: torii_proto::PlayerAchievementProgress) -> Self {
        let task_progress: Vec<TaskProgress> =
            val.task_progress.into_iter().map(|t| t.into()).collect();

        PlayerAchievementProgress {
            achievement: val.achievement.into(),
            task_progress,
            completed: val.completed,
            progress_percentage: val.progress_percentage,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerAchievementStats {
    pub total_points: u32,
    pub completed_achievements: u32,
    pub total_achievements: u32,
    pub completion_percentage: f64,
    pub last_achievement_at: Option<u64>,
    pub created_at: u64,
    pub updated_at: u64,
}

impl From<torii_proto::PlayerAchievementStats> for PlayerAchievementStats {
    fn from(val: torii_proto::PlayerAchievementStats) -> Self {
        PlayerAchievementStats {
            total_points: val.total_points,
            completed_achievements: val.completed_achievements,
            total_achievements: val.total_achievements,
            completion_percentage: val.completion_percentage,
            last_achievement_at: val.last_achievement_at.map(|t| t.timestamp() as u64),
            created_at: val.created_at.timestamp() as u64,
            updated_at: val.updated_at.timestamp() as u64,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerAchievementEntry {
    pub player_address: FieldElement,
    pub stats: PlayerAchievementStats,
    pub achievements: Vec<PlayerAchievementProgress>,
}

impl From<torii_proto::PlayerAchievementEntry> for PlayerAchievementEntry {
    fn from(val: torii_proto::PlayerAchievementEntry) -> Self {
        let achievements: Vec<PlayerAchievementProgress> =
            val.achievements.into_iter().map(|a| a.into()).collect();

        PlayerAchievementEntry {
            player_address: felt_to_field_element(val.player_address),
            stats: val.stats.into(),
            achievements,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerAchievementQuery {
    pub world_addresses: Vec<FieldElement>,
    pub namespaces: Vec<String>,
    pub player_addresses: Vec<FieldElement>,
    pub pagination: Pagination,
}

impl From<PlayerAchievementQuery> for torii_proto::PlayerAchievementQuery {
    fn from(val: PlayerAchievementQuery) -> Self {
        torii_proto::PlayerAchievementQuery {
            world_addresses: val
                .world_addresses
                .into_iter()
                .map(|a| field_element_to_felt(&a).unwrap())
                .collect(),
            namespaces: val.namespaces,
            player_addresses: val
                .player_addresses
                .into_iter()
                .map(|a| field_element_to_felt(&a).unwrap())
                .collect(),
            pagination: val.pagination.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AchievementProgression {
    pub id: String,
    pub achievement_id: String,
    pub task_id: String,
    pub world_address: FieldElement,
    pub namespace: String,
    pub player_id: FieldElement,
    pub count: u32,
    pub completed: bool,
    pub completed_at: Option<u64>,
    pub created_at: u64,
    pub updated_at: u64,
}

impl From<torii_proto::AchievementProgression> for AchievementProgression {
    fn from(val: torii_proto::AchievementProgression) -> Self {
        AchievementProgression {
            id: val.id,
            achievement_id: val.achievement_id,
            task_id: val.task_id,
            world_address: felt_to_field_element(val.world_address),
            namespace: val.namespace,
            player_id: felt_to_field_element(val.player_id),
            count: val.count,
            completed: val.completed,
            completed_at: val.completed_at.map(|t| t.timestamp() as u64),
            created_at: val.created_at.timestamp() as u64,
            updated_at: val.updated_at.timestamp() as u64,
        }
    }
}
