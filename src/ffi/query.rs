#[diplomat::bridge]
pub mod ffi {
    use diplomat_runtime::DiplomatStr;

    use crate::ffi::error::ffi::DojoError;
    use crate::ffi::types::ffi::FieldElement;

    /// Query clause for filtering entities
    #[diplomat::opaque]
    pub struct Clause {
        pub(crate) inner: torii_proto::Clause,
    }

    /// Keys clause for filtering by entity keys
    #[diplomat::opaque]
    pub struct KeysClause {
        pub(crate) keys: Vec<starknet_crypto::Felt>,
        pub(crate) pattern_matching: torii_proto::PatternMatching,
        pub(crate) models: Vec<String>,
    }

    impl KeysClause {
        /// Creates a new keys clause
        pub fn new() -> Box<KeysClause> {
            Box::new(KeysClause {
                keys: Vec::new(),
                pattern_matching: torii_proto::PatternMatching::FixedLen,
                models: Vec::new(),
            })
        }

        /// Adds a key to filter by
        pub fn add_key(&mut self, key: &FieldElement) {
            self.keys.push(key.0);
        }

        /// Adds a model to filter by
        pub fn add_model(&mut self, model: &DiplomatStr) -> Result<(), Box<DojoError>> {
            let s = std::str::from_utf8(model)?;
            self.models.push(s.to_string());
            Ok(())
        }
    }

    /// Entity query for retrieving entities from Torii
    #[diplomat::opaque]
    pub struct EntityQuery {
        pub(crate) clause: Option<torii_proto::Clause>,
        pub(crate) limit: u32,
        pub(crate) offset: u32,
    }

    impl EntityQuery {
        /// Creates a new entity query
        pub fn new() -> Box<EntityQuery> {
            Box::new(EntityQuery {
                clause: None,
                limit: 100,
                offset: 0,
            })
        }

        /// Sets the limit for the query
        pub fn set_limit(&mut self, limit: u32) {
            self.limit = limit;
        }

        /// Sets the offset for the query
        pub fn set_offset(&mut self, offset: u32) {
            self.offset = offset;
        }
    }

    /// Model query for retrieving model definitions
    #[diplomat::opaque]
    pub struct ModelQuery {
        pub(crate) model_names: Vec<String>,
    }

    impl ModelQuery {
        /// Creates a new model query
        pub fn new() -> Box<ModelQuery> {
            Box::new(ModelQuery {
                model_names: Vec::new(),
            })
        }

        /// Adds a model name to query
        pub fn add_model(&mut self, model_name: &DiplomatStr) -> Result<(), Box<DojoError>> {
            let s = std::str::from_utf8(model_name)?;
            self.model_names.push(s.to_string());
            Ok(())
        }
    }
}

