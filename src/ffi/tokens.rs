
#[diplomat::bridge]
pub mod ffi {
    use diplomat_runtime::{DiplomatStr, DiplomatWrite};
    use std::fmt::Write;
    use crate::ffi::error::ffi::DojoError;

    /// Represents a token (ERC20, ERC721, or ERC1155)
    #[diplomat::opaque]
    pub struct Token {
        pub(crate) inner: torii_proto::Token,
    }

    impl Token {
        /// Gets the contract address (hex)
        pub fn contract_address(&self, write: &mut DiplomatWrite) {
            write!(write, "{:#x}", self.inner.contract_address).unwrap();
        }

        /// Gets the token name
        pub fn name(&self, write: &mut DiplomatWrite) {
            write!(write, "{}", self.inner.name).unwrap();
        }

        /// Gets the token symbol
        pub fn symbol(&self, write: &mut DiplomatWrite) {
            write!(write, "{}", self.inner.symbol).unwrap();
        }

        /// Gets the token decimals
        pub fn decimals(&self) -> u8 {
            self.inner.decimals
        }

        /// Gets the metadata as JSON string
        pub fn metadata(&self, write: &mut DiplomatWrite) {
            write!(write, "{}", self.inner.metadata).unwrap();
        }

        /// Creates a token from JSON
        pub fn from_json(json: &DiplomatStr) -> Result<Box<Token>, Box<DojoError>> {
            let s = std::str::from_utf8(json)?;
            let inner: torii_proto::Token = serde_json::from_str(s)?;
            Ok(Box::new(Token { inner }))
        }

        /// Serializes the token to JSON
        pub fn to_json(&self, write: &mut DiplomatWrite) -> Result<(), Box<DojoError>> {
            let json = serde_json::to_string(&self.inner)?;
            write!(write, "{}", json).unwrap();
            Ok(())
        }
    }

    /// Represents a token balance for an account
    #[diplomat::opaque]
    pub struct TokenBalance {
        pub(crate) inner: torii_proto::TokenBalance,
    }

    impl TokenBalance {
        /// Gets the account address (hex)
        pub fn account_address(&self, write: &mut DiplomatWrite) {
            write!(write, "{:#x}", self.inner.account_address).unwrap();
        }

        /// Gets the contract address (hex)
        pub fn contract_address(&self, write: &mut DiplomatWrite) {
            write!(write, "{:#x}", self.inner.contract_address).unwrap();
        }

        /// Gets the balance as a string
        pub fn balance(&self, write: &mut DiplomatWrite) {
            write!(write, "{}", self.inner.balance).unwrap();
        }

        /// Creates a token balance from JSON
        pub fn from_json(json: &DiplomatStr) -> Result<Box<TokenBalance>, Box<DojoError>> {
            let s = std::str::from_utf8(json)?;
            let inner: torii_proto::TokenBalance = serde_json::from_str(s)?;
            Ok(Box::new(TokenBalance { inner }))
        }

        /// Serializes the token balance to JSON
        pub fn to_json(&self, write: &mut DiplomatWrite) -> Result<(), Box<DojoError>> {
            let json = serde_json::to_string(&self.inner)?;
            write!(write, "{}", json).unwrap();
            Ok(())
        }
    }

    /// Represents a token transfer event
    #[diplomat::opaque]
    pub struct TokenTransfer {
        pub(crate) inner: torii_proto::TokenTransfer,
    }

    impl TokenTransfer {
        /// Gets the from address (hex)
        pub fn from_address(&self, write: &mut DiplomatWrite) {
            write!(write, "{:#x}", self.inner.from_address).unwrap();
        }

        /// Gets the to address (hex)
        pub fn to_address(&self, write: &mut DiplomatWrite) {
            write!(write, "{:#x}", self.inner.to_address).unwrap();
        }

        /// Gets the contract address (hex)
        pub fn contract_address(&self, write: &mut DiplomatWrite) {
            write!(write, "{:#x}", self.inner.contract_address).unwrap();
        }

        /// Gets the amount as a string
        pub fn amount(&self, write: &mut DiplomatWrite) {
            write!(write, "{}", self.inner.amount).unwrap();
        }

        /// Gets the executed_at timestamp
        pub fn executed_at(&self) -> u64 {
            self.inner.executed_at.timestamp() as u64
        }

        /// Creates a token transfer from JSON
        pub fn from_json(json: &DiplomatStr) -> Result<Box<TokenTransfer>, Box<DojoError>> {
            let s = std::str::from_utf8(json)?;
            let inner: torii_proto::TokenTransfer = serde_json::from_str(s)?;
            Ok(Box::new(TokenTransfer { inner }))
        }

        /// Serializes the token transfer to JSON
        pub fn to_json(&self, write: &mut DiplomatWrite) -> Result<(), Box<DojoError>> {
            let json = serde_json::to_string(&self.inner)?;
            write!(write, "{}", json).unwrap();
            Ok(())
        }
    }

    /// Represents a token contract
    #[diplomat::opaque]
    pub struct TokenContract {
        pub(crate) inner: torii_proto::TokenContract,
    }

    impl TokenContract {
        /// Gets the contract address (hex)
        pub fn contract_address(&self, write: &mut DiplomatWrite) {
            write!(write, "{:#x}", self.inner.contract_address).unwrap();
        }

        /// Gets the contract type as string
        pub fn contract_type(&self, write: &mut DiplomatWrite) {
            write!(write, "{}", self.inner.r#type).unwrap();
        }

        /// Creates a token contract from JSON
        pub fn from_json(json: &DiplomatStr) -> Result<Box<TokenContract>, Box<DojoError>> {
            let s = std::str::from_utf8(json)?;
            let inner: torii_proto::TokenContract = serde_json::from_str(s)?;
            Ok(Box::new(TokenContract { inner }))
        }

        /// Serializes the token contract to JSON
        pub fn to_json(&self, write: &mut DiplomatWrite) -> Result<(), Box<DojoError>> {
            let json = serde_json::to_string(&self.inner)?;
            write!(write, "{}", json).unwrap();
            Ok(())
        }
    }
}

