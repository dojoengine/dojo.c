
#[diplomat::bridge]
pub mod ffi {
    use diplomat_runtime::{DiplomatStr, DiplomatWrite};
    use std::fmt::Write;
    use crate::ffi::error::ffi::DojoError;

    /// Represents a contract
    #[diplomat::opaque]
    pub struct Contract {
        pub(crate) inner: torii_proto::Contract,
    }

    impl Contract {
        /// Gets the contract address (hex)
        pub fn contract_address(&self, write: &mut DiplomatWrite) {
            write!(write, "{:#x}", self.inner.contract_address).unwrap();
        }

        /// Gets the contract type as string
        pub fn contract_type(&self, write: &mut DiplomatWrite) {
            let type_str = match self.inner.contract_type {
                torii_proto::ContractType::WORLD => "WORLD",
                torii_proto::ContractType::ERC20 => "ERC20",
                torii_proto::ContractType::ERC721 => "ERC721",
                torii_proto::ContractType::ERC1155 => "ERC1155",
                torii_proto::ContractType::UDC => "UDC",
                torii_proto::ContractType::OTHER => "OTHER",
            };
            write!(write, "{}", type_str).unwrap();
        }

        /// Gets the head block number (if any)
        pub fn head(&self) -> u64 {
            self.inner.head.unwrap_or(0)
        }

        /// Gets the TPS (transactions per second) if available
        pub fn tps(&self) -> u64 {
            self.inner.tps.unwrap_or(0)
        }

        /// Gets the created_at timestamp
        pub fn created_at(&self) -> u64 {
            self.inner.created_at.timestamp() as u64
        }

        /// Gets the updated_at timestamp
        pub fn updated_at(&self) -> u64 {
            self.inner.updated_at.timestamp() as u64
        }

        /// Creates a contract from JSON
        pub fn from_json(json: &DiplomatStr) -> Result<Box<Contract>, Box<DojoError>> {
            let s = std::str::from_utf8(json)?;
            let inner: torii_proto::Contract = serde_json::from_str(s)?;
            Ok(Box::new(Contract { inner }))
        }

        /// Serializes the contract to JSON
        pub fn to_json(&self, write: &mut DiplomatWrite) -> Result<(), Box<DojoError>> {
            let json = serde_json::to_string(&self.inner)?;
            write!(write, "{}", json).unwrap();
            Ok(())
        }
    }
}

