
#[diplomat::bridge]
pub mod ffi {
    use diplomat_runtime::{DiplomatStr, DiplomatWrite};
    use std::fmt::Write;
    use crate::ffi::error::ffi::DojoError;

    /// Represents an achievement
    #[diplomat::opaque]
    pub struct Achievement {
        pub(crate) inner: torii_proto::Achievement,
    }

    impl Achievement {
        /// Gets the achievement ID
        pub fn id(&self, write: &mut DiplomatWrite) {
            write!(write, "{}", self.inner.id).unwrap();
        }

        /// Gets the world address (hex)
        pub fn world_address(&self, write: &mut DiplomatWrite) {
            write!(write, "{:#x}", self.inner.world_address).unwrap();
        }

        /// Gets the namespace
        pub fn namespace(&self, write: &mut DiplomatWrite) {
            write!(write, "{}", self.inner.namespace).unwrap();
        }

        /// Gets the achievement title
        pub fn title(&self, write: &mut DiplomatWrite) {
            write!(write, "{}", self.inner.title).unwrap();
        }

        /// Gets the achievement description
        pub fn description(&self, write: &mut DiplomatWrite) {
            write!(write, "{}", self.inner.description).unwrap();
        }

        /// Gets the hidden flag
        pub fn hidden(&self) -> bool {
            self.inner.hidden
        }

        /// Gets the icon URI
        pub fn icon(&self, write: &mut DiplomatWrite) {
            write!(write, "{}", self.inner.icon).unwrap();
        }

        /// Gets the points for this achievement
        pub fn points(&self) -> u32 {
            self.inner.points
        }

        /// Creates an achievement from JSON
        pub fn from_json(json: &DiplomatStr) -> Result<Box<Achievement>, Box<DojoError>> {
            let s = std::str::from_utf8(json)?;
            let inner: torii_proto::Achievement = serde_json::from_str(s)?;
            Ok(Box::new(Achievement { inner }))
        }

        /// Serializes the achievement to JSON
        pub fn to_json(&self, write: &mut DiplomatWrite) -> Result<(), Box<DojoError>> {
            let json = serde_json::to_string(&self.inner)?;
            write!(write, "{}", json).unwrap();
            Ok(())
        }
    }

    /// Represents player achievement progress
    #[diplomat::opaque]
    pub struct PlayerAchievementEntry {
        pub(crate) inner: torii_proto::PlayerAchievementEntry,
    }

    impl PlayerAchievementEntry {
        /// Gets the player address (hex)
        pub fn player_address(&self, write: &mut DiplomatWrite) {
            write!(write, "{:#x}", self.inner.player_address).unwrap();
        }

        /// Gets the total points
        pub fn total_points(&self) -> u32 {
            self.inner.stats.total_points
        }

        /// Gets the completed achievements count
        pub fn completed_achievements(&self) -> u32 {
            self.inner.stats.completed_achievements
        }

        /// Gets the total achievements count
        pub fn total_achievements(&self) -> u32 {
            self.inner.stats.total_achievements
        }

        /// Gets the completion percentage
        pub fn completion_percentage(&self) -> f64 {
            self.inner.stats.completion_percentage
        }

        /// Gets the achievements count
        pub fn achievements_count(&self) -> u32 {
            self.inner.achievements.len() as u32
        }

        /// Gets the updated_at timestamp
        pub fn updated_at(&self) -> u64 {
            self.inner.stats.updated_at.timestamp() as u64
        }

        /// Creates a player achievement entry from JSON
        pub fn from_json(json: &DiplomatStr) -> Result<Box<PlayerAchievementEntry>, Box<DojoError>> {
            let s = std::str::from_utf8(json)?;
            let inner: torii_proto::PlayerAchievementEntry = serde_json::from_str(s)?;
            Ok(Box::new(PlayerAchievementEntry { inner }))
        }

        /// Serializes the player achievement entry to JSON
        pub fn to_json(&self, write: &mut DiplomatWrite) -> Result<(), Box<DojoError>> {
            let json = serde_json::to_string(&self.inner)?;
            write!(write, "{}", json).unwrap();
            Ok(())
        }
    }
}

