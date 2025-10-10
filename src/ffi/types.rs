#[diplomat::bridge]
pub mod ffi {
    use crypto_bigint::Encoding;
    use diplomat_runtime::{DiplomatStr, DiplomatWrite};
    use starknet_crypto::Felt;
    use std::fmt::Write;

    use crate::ffi::error::ffi::{DojoError, ErrorType};

    /// Represents a Starknet field element (Felt)
    #[diplomat::opaque]
    pub struct FieldElement(pub Felt);

    impl FieldElement {
        /// Creates a new FieldElement from a hexadecimal string
        pub fn new_from_hex(hex: &DiplomatStr) -> Result<Box<FieldElement>, Box<DojoError>> {
            let s = std::str::from_utf8(hex)?;
            let felt = Felt::from_hex(s).map_err(|e| {
                DojoError::new(ErrorType::ParseError, &format!("Invalid felt hex: {}", e))
            })?;
            Ok(Box::new(FieldElement(felt)))
        }

        /// Creates a new FieldElement from big-endian bytes
        pub fn new_from_bytes(bytes: &[u8]) -> Box<FieldElement> {
            Box::new(FieldElement(Felt::from_bytes_be_slice(bytes)))
        }

        /// Returns the field element as a hexadecimal string
        pub fn to_hex(&self, result: &mut DiplomatWrite) {
            write!(result, "{:#x}", self.0).unwrap();
        }

        /// Returns the field element as bytes (big-endian)
        pub fn to_bytes(&self, result: &mut [u8]) {
            let bytes = self.0.to_bytes_be();
            result[..bytes.len()].copy_from_slice(&bytes);
        }
    }

    /// Represents a 256-bit unsigned integer
    #[diplomat::opaque]
    pub struct U256(pub crypto_bigint::U256);

    impl U256 {
        /// Creates a new U256 from a hexadecimal string
        pub fn new_from_hex(hex: &DiplomatStr) -> Result<Box<U256>, Box<DojoError>> {
            let s = std::str::from_utf8(hex)?;
            let trimmed = s.trim_start_matches("0x");
            let value = crypto_bigint::U256::from_be_hex(trimmed);
            Ok(Box::new(U256(value)))
        }

        /// Creates a new U256 from big-endian bytes
        pub fn new_from_bytes(bytes: &[u8]) -> Result<Box<U256>, Box<DojoError>> {
            if bytes.len() != 32 {
                return Err(DojoError::new(
                    ErrorType::InvalidInput,
                    "U256 requires exactly 32 bytes",
                ));
            }
            let mut arr = [0u8; 32];
            arr.copy_from_slice(bytes);
            Ok(Box::new(U256(crypto_bigint::U256::from_be_bytes(arr))))
        }

        /// Returns the U256 as a hexadecimal string
        pub fn to_hex(&self, result: &mut DiplomatWrite) {
            write!(result, "0x{:x}", self.0).unwrap();
        }

        /// Returns the U256 as bytes (big-endian)
        pub fn to_bytes(&self, result: &mut [u8]) {
            let bytes = self.0.to_be_bytes();
            result[..32].copy_from_slice(&bytes);
        }
    }

    /// Represents a Starknet call
    #[diplomat::opaque]
    pub struct Call(pub starknet::core::types::Call);

    impl Call {
        /// Creates a new Call
        pub fn new(
            to: &FieldElement,
            selector: &FieldElement,
        ) -> Box<Call> {
            Box::new(Call(starknet::core::types::Call {
                to: to.0,
                selector: selector.0,
                calldata: Vec::new(),
            }))
        }

        /// Adds a field element to the calldata
        pub fn push_calldata(&mut self, felt: &FieldElement) {
            self.0.calldata.push(felt.0);
        }

        /// Creates a call from selector name
        pub fn new_from_selector_name(
            to: &FieldElement,
            selector_name: &DiplomatStr,
        ) -> Result<Box<Call>, Box<DojoError>> {
            let name = std::str::from_utf8(selector_name)?;
            let selector = starknet::core::utils::get_selector_from_name(name)
                .map_err(|e| DojoError::new(ErrorType::ParseError, &format!("Invalid selector: {}", e)))?;
            
            Ok(Box::new(Call(starknet::core::types::Call {
                to: to.0,
                selector,
                calldata: Vec::new(),
            })))
        }
    }

    /// List of calls for batch transactions
    #[diplomat::opaque]
    pub struct CallList {
        pub(crate) calls: Vec<starknet::core::types::Call>,
    }

    impl CallList {
        /// Creates a new empty call list
        pub fn new() -> Box<CallList> {
            Box::new(CallList { calls: Vec::new() })
        }

        /// Adds a call to the list
        pub fn add_call(&mut self, call: &Call) {
            self.calls.push(call.0.clone());
        }

        /// Returns the number of calls in the list
        pub fn len(&self) -> usize {
            self.calls.len()
        }
    }

    /// Represents a signature (r, s pair)
    #[diplomat::opaque]
    pub struct Signature {
        pub(crate) r: Felt,
        pub(crate) s: Felt,
    }

    impl Signature {
        /// Creates a new signature from r and s components
        pub fn new(r: &FieldElement, s: &FieldElement) -> Box<Signature> {
            Box::new(Signature { r: r.0, s: s.0 })
        }

        /// Gets the r component
        pub fn r(&self, result: &mut DiplomatWrite) {
            write!(result, "{:#x}", self.r).unwrap();
        }

            /// Gets the s component
        pub fn s(&self, result: &mut DiplomatWrite) {
            write!(result, "{:#x}", self.s).unwrap();
        }
    }

    use crate::ffi::enums::ffi::{BlockTag, OrderDirection, PaginationDirection};

    /// Block identifier (hash, number, or tag)
    #[diplomat::opaque]
    pub struct BlockId(pub(crate) starknet::core::types::BlockId);

    impl BlockId {
        /// Creates a BlockId from a block hash
        pub fn from_hash(hash: &FieldElement) -> Box<BlockId> {
            Box::new(BlockId(starknet::core::types::BlockId::Hash(hash.0)))
        }

        /// Creates a BlockId from a block number
        pub fn from_number(number: u64) -> Box<BlockId> {
            Box::new(BlockId(starknet::core::types::BlockId::Number(number)))
        }

        /// Creates a BlockId from a block tag
        pub fn from_tag(tag: BlockTag) -> Box<BlockId> {
            Box::new(BlockId(starknet::core::types::BlockId::Tag(tag.into())))
        }
    }

    /// Ordering specification for query results
    #[diplomat::opaque]
    pub struct OrderBy {
        pub(crate) field: String,
        pub(crate) direction: torii_proto::OrderDirection,
    }

    impl OrderBy {
        /// Creates a new OrderBy specification
        pub fn new(field: &DiplomatStr, direction: OrderDirection) -> Result<Box<OrderBy>, Box<DojoError>> {
            let field = std::str::from_utf8(field)?.to_string();
            Ok(Box::new(OrderBy {
                field,
                direction: direction.into(),
            }))
        }
    }

    /// Pagination configuration for queries
    #[diplomat::opaque]
    pub struct Pagination {
        pub(crate) cursor: Option<String>,
        pub(crate) limit: Option<u32>,
        pub(crate) direction: torii_proto::PaginationDirection,
        pub(crate) order_by: Vec<torii_proto::OrderBy>,
    }

    impl Pagination {
        /// Creates a new Pagination with default values
        pub fn new() -> Box<Pagination> {
            Box::new(Pagination {
                cursor: None,
                limit: Some(100),
                direction: torii_proto::PaginationDirection::Forward,
                order_by: Vec::new(),
            })
        }

        /// Sets the cursor for pagination
        pub fn set_cursor(&mut self, cursor: &DiplomatStr) -> Result<(), Box<DojoError>> {
            let s = std::str::from_utf8(cursor)?;
            self.cursor = Some(s.to_string());
            Ok(())
        }

        /// Sets the limit for pagination
        pub fn set_limit(&mut self, limit: u32) {
            self.limit = Some(limit);
        }

        /// Sets the direction for pagination
        pub fn set_direction(&mut self, direction: PaginationDirection) {
            self.direction = direction.into();
        }

        /// Adds an ordering specification
        pub fn add_order_by(&mut self, order_by: &OrderBy) {
            self.order_by.push(torii_proto::OrderBy {
                field: order_by.field.clone(),
                direction: order_by.direction.clone(),
            });
        }
    }
}


