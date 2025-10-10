
#[diplomat::bridge]
pub mod ffi {
    use diplomat_runtime::{DiplomatStr, DiplomatWrite};
    use std::fmt::Write;
    use crate::ffi::error::ffi::DojoError;

    /// Represents an activity
    #[diplomat::opaque]
    pub struct Activity {
        pub(crate) inner: torii_proto::Activity,
    }

    impl Activity {
        /// Gets the activity ID
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

        /// Gets the caller address (hex)
        pub fn caller_address(&self, write: &mut DiplomatWrite) {
            write!(write, "{:#x}", self.inner.caller_address).unwrap();
        }

        /// Gets the session start timestamp
        pub fn session_start(&self) -> u64 {
            self.inner.session_start.timestamp() as u64
        }

        /// Gets the session end timestamp
        pub fn session_end(&self) -> u64 {
            self.inner.session_end.timestamp() as u64
        }

        /// Gets the action count
        pub fn action_count(&self) -> u32 {
            self.inner.action_count
        }

        /// Gets the updated_at timestamp
        pub fn updated_at(&self) -> u64 {
            self.inner.updated_at.timestamp() as u64
        }

        /// Creates an activity from JSON
        pub fn from_json(json: &DiplomatStr) -> Result<Box<Activity>, Box<DojoError>> {
            let s = std::str::from_utf8(json)?;
            let inner: torii_proto::Activity = serde_json::from_str(s)?;
            Ok(Box::new(Activity { inner }))
        }

        /// Serializes the activity to JSON
        pub fn to_json(&self, write: &mut DiplomatWrite) -> Result<(), Box<DojoError>> {
            let json = serde_json::to_string(&self.inner)?;
            write!(write, "{}", json).unwrap();
            Ok(())
        }
    }

    /// Represents an aggregation entry
    #[diplomat::opaque]
    pub struct AggregationEntry {
        pub(crate) inner: torii_proto::AggregationEntry,
    }

    impl AggregationEntry {
        /// Gets the aggregation ID
        pub fn id(&self, write: &mut DiplomatWrite) {
            write!(write, "{}", self.inner.id).unwrap();
        }

        /// Gets the aggregator ID
        pub fn aggregator_id(&self, write: &mut DiplomatWrite) {
            write!(write, "{}", self.inner.aggregator_id).unwrap();
        }

        /// Gets the entity ID
        pub fn entity_id(&self, write: &mut DiplomatWrite) {
            write!(write, "{}", self.inner.entity_id).unwrap();
        }

        /// Gets the model ID
        pub fn model_id(&self, write: &mut DiplomatWrite) {
            write!(write, "{}", self.inner.model_id).unwrap();
        }

        /// Gets the aggregation value
        pub fn value(&self, write: &mut DiplomatWrite) {
            write!(write, "{}", self.inner.value).unwrap();
        }

        /// Gets the display value
        pub fn display_value(&self, write: &mut DiplomatWrite) {
            write!(write, "{}", self.inner.display_value).unwrap();
        }

        /// Gets the position
        pub fn position(&self) -> u64 {
            self.inner.position
        }

        /// Gets the created_at timestamp
        pub fn created_at(&self) -> u64 {
            self.inner.created_at.timestamp() as u64
        }

        /// Gets the updated_at timestamp
        pub fn updated_at(&self) -> u64 {
            self.inner.updated_at.timestamp() as u64
        }

        /// Creates an aggregation entry from JSON
        pub fn from_json(json: &DiplomatStr) -> Result<Box<AggregationEntry>, Box<DojoError>> {
            let s = std::str::from_utf8(json)?;
            let inner: torii_proto::AggregationEntry = serde_json::from_str(s)?;
            Ok(Box::new(AggregationEntry { inner }))
        }

        /// Serializes the aggregation entry to JSON
        pub fn to_json(&self, write: &mut DiplomatWrite) -> Result<(), Box<DojoError>> {
            let json = serde_json::to_string(&self.inner)?;
            write!(write, "{}", json).unwrap();
            Ok(())
        }
    }
}

