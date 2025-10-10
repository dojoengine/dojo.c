
#[diplomat::bridge]
pub mod ffi {
    use diplomat_runtime::{DiplomatStr, DiplomatWrite};
    use std::fmt::Write;
    use crate::ffi::error::ffi::DojoError;

    /// Represents a Dojo event message
    #[diplomat::opaque]
    pub struct Event {
        pub(crate) inner: torii_proto::Event,
    }

    impl Event {
        /// Gets the keys as JSON array string
        pub fn keys(&self, write: &mut DiplomatWrite) {
            write!(write, "[").unwrap();
            for (i, key) in self.inner.keys.iter().enumerate() {
                if i > 0 {
                    write!(write, ",").unwrap();
                }
                write!(write, "\"{:#x}\"", key).unwrap();
            }
            write!(write, "]").unwrap();
        }

        /// Gets the data as JSON array string
        pub fn data(&self, write: &mut DiplomatWrite) {
            write!(write, "[").unwrap();
            for (i, d) in self.inner.data.iter().enumerate() {
                if i > 0 {
                    write!(write, ",").unwrap();
                }
                write!(write, "\"{:#x}\"", d).unwrap();
            }
            write!(write, "]").unwrap();
        }

        /// Gets the transaction hash (hex)
        pub fn transaction_hash(&self, write: &mut DiplomatWrite) {
            write!(write, "{:#x}", self.inner.transaction_hash).unwrap();
        }

        /// Creates an event from JSON
        pub fn from_json(json: &DiplomatStr) -> Result<Box<Event>, Box<DojoError>> {
            let s = std::str::from_utf8(json)?;
            let inner: torii_proto::Event = serde_json::from_str(s)?;
            Ok(Box::new(Event { inner }))
        }

        /// Serializes the event to JSON
        pub fn to_json(&self, write: &mut DiplomatWrite) -> Result<(), Box<DojoError>> {
            let json = serde_json::to_string(&self.inner)?;
            write!(write, "{}", json).unwrap();
            Ok(())
        }
    }

    /// Represents a transaction
    #[diplomat::opaque]
    pub struct Transaction {
        pub(crate) inner: torii_proto::Transaction,
    }

    impl Transaction {
        /// Gets the transaction hash (hex)
        pub fn transaction_hash(&self, write: &mut DiplomatWrite) {
            write!(write, "{:#x}", self.inner.transaction_hash).unwrap();
        }

        /// Gets the sender address (hex)
        pub fn sender_address(&self, write: &mut DiplomatWrite) {
            write!(write, "{:#x}", self.inner.sender_address).unwrap();
        }

        /// Gets the max fee as string
        pub fn max_fee(&self, write: &mut DiplomatWrite) {
            write!(write, "{}", self.inner.max_fee).unwrap();
        }

        /// Gets the signature as JSON array string
        pub fn signature(&self, write: &mut DiplomatWrite) {
            write!(write, "[").unwrap();
            for (i, sig) in self.inner.signature.iter().enumerate() {
                if i > 0 {
                    write!(write, ",").unwrap();
                }
                write!(write, "\"{:#x}\"", sig).unwrap();
            }
            write!(write, "]").unwrap();
        }

        /// Gets the nonce as string
        pub fn nonce(&self, write: &mut DiplomatWrite) {
            write!(write, "{}", self.inner.nonce).unwrap();
        }

        /// Gets the block timestamp
        pub fn block_timestamp(&self) -> u64 {
            self.inner.block_timestamp.timestamp() as u64
        }

        /// Gets the block number
        pub fn block_number(&self) -> u64 {
            self.inner.block_number
        }

        /// Creates a transaction from JSON
        pub fn from_json(json: &DiplomatStr) -> Result<Box<Transaction>, Box<DojoError>> {
            let s = std::str::from_utf8(json)?;
            let inner: torii_proto::Transaction = serde_json::from_str(s)?;
            Ok(Box::new(Transaction { inner }))
        }

        /// Serializes the transaction to JSON
        pub fn to_json(&self, write: &mut DiplomatWrite) -> Result<(), Box<DojoError>> {
            let json = serde_json::to_string(&self.inner)?;
            write!(write, "{}", json).unwrap();
            Ok(())
        }
    }
}

