#[diplomat::bridge]
pub mod ffi {
    /// Block tag for identifying specific blocks
    pub enum BlockTag {
        Latest,
        PreConfirmed,
    }

    /// Type of contract
    #[allow(clippy::upper_case_acronyms)]
    pub enum ContractType {
        WORLD,
        ERC20,
        ERC721,
        ERC1155,
        UDC,
        OTHER,
    }

    /// Direction for pagination
    pub enum PaginationDirection {
        Forward,
        Backward,
    }

    /// Direction for ordering results
    pub enum OrderDirection {
        Asc,
        Desc,
    }

    /// Pattern matching mode for key queries
    pub enum PatternMatching {
        FixedLen,
        VariableLen,
    }

    /// Logical operator for combining clauses
    pub enum LogicalOperator {
        And,
        Or,
    }

    /// Comparison operators for member clauses
    pub enum ComparisonOperator {
        Eq,
        Neq,
        Gt,
        Gte,
        Lt,
        Lte,
        In,
        NotIn,
        Contains,
        ContainsAll,
        ContainsAny,
        ArrayLengthEq,
        ArrayLengthGt,
        ArrayLengthLt,
    }

    /// Type of call in a transaction
    pub enum CallType {
        Execute,
        ExecuteFromOutside,
    }

    // Conversion implementations for torii_proto types
    impl From<BlockTag> for starknet::core::types::BlockTag {
        fn from(val: BlockTag) -> Self {
            match val {
                BlockTag::Latest => starknet::core::types::BlockTag::Latest,
                BlockTag::PreConfirmed => starknet::core::types::BlockTag::PreConfirmed,
            }
        }
    }

    impl From<starknet::core::types::BlockTag> for BlockTag {
        fn from(val: starknet::core::types::BlockTag) -> Self {
            match val {
                starknet::core::types::BlockTag::Latest => BlockTag::Latest,
                starknet::core::types::BlockTag::PreConfirmed => BlockTag::PreConfirmed,
                _ => BlockTag::Latest, // Default for other tags
            }
        }
    }

    impl From<ContractType> for torii_proto::ContractType {
        fn from(val: ContractType) -> Self {
            match val {
                ContractType::WORLD => torii_proto::ContractType::WORLD,
                ContractType::ERC20 => torii_proto::ContractType::ERC20,
                ContractType::ERC721 => torii_proto::ContractType::ERC721,
                ContractType::ERC1155 => torii_proto::ContractType::ERC1155,
                ContractType::UDC => torii_proto::ContractType::UDC,
                ContractType::OTHER => torii_proto::ContractType::OTHER,
            }
        }
    }

    impl From<torii_proto::ContractType> for ContractType {
        fn from(val: torii_proto::ContractType) -> Self {
            match val {
                torii_proto::ContractType::WORLD => ContractType::WORLD,
                torii_proto::ContractType::ERC20 => ContractType::ERC20,
                torii_proto::ContractType::ERC721 => ContractType::ERC721,
                torii_proto::ContractType::ERC1155 => ContractType::ERC1155,
                torii_proto::ContractType::UDC => ContractType::UDC,
                torii_proto::ContractType::OTHER => ContractType::OTHER,
            }
        }
    }

    impl From<PaginationDirection> for torii_proto::PaginationDirection {
        fn from(val: PaginationDirection) -> Self {
            match val {
                PaginationDirection::Forward => torii_proto::PaginationDirection::Forward,
                PaginationDirection::Backward => torii_proto::PaginationDirection::Backward,
            }
        }
    }

    impl From<torii_proto::PaginationDirection> for PaginationDirection {
        fn from(val: torii_proto::PaginationDirection) -> Self {
            match val {
                torii_proto::PaginationDirection::Forward => PaginationDirection::Forward,
                torii_proto::PaginationDirection::Backward => PaginationDirection::Backward,
            }
        }
    }

    impl From<OrderDirection> for torii_proto::OrderDirection {
        fn from(val: OrderDirection) -> Self {
            match val {
                OrderDirection::Asc => torii_proto::OrderDirection::Asc,
                OrderDirection::Desc => torii_proto::OrderDirection::Desc,
            }
        }
    }

    impl From<torii_proto::OrderDirection> for OrderDirection {
        fn from(val: torii_proto::OrderDirection) -> Self {
            match val {
                torii_proto::OrderDirection::Asc => OrderDirection::Asc,
                torii_proto::OrderDirection::Desc => OrderDirection::Desc,
            }
        }
    }

    impl From<PatternMatching> for torii_proto::PatternMatching {
        fn from(val: PatternMatching) -> Self {
            match val {
                PatternMatching::FixedLen => torii_proto::PatternMatching::FixedLen,
                PatternMatching::VariableLen => torii_proto::PatternMatching::VariableLen,
            }
        }
    }

    impl From<torii_proto::PatternMatching> for PatternMatching {
        fn from(val: torii_proto::PatternMatching) -> Self {
            match val {
                torii_proto::PatternMatching::FixedLen => PatternMatching::FixedLen,
                torii_proto::PatternMatching::VariableLen => PatternMatching::VariableLen,
            }
        }
    }

    impl From<LogicalOperator> for torii_proto::LogicalOperator {
        fn from(val: LogicalOperator) -> Self {
            match val {
                LogicalOperator::And => torii_proto::LogicalOperator::And,
                LogicalOperator::Or => torii_proto::LogicalOperator::Or,
            }
        }
    }

    impl From<torii_proto::LogicalOperator> for LogicalOperator {
        fn from(val: torii_proto::LogicalOperator) -> Self {
            match val {
                torii_proto::LogicalOperator::And => LogicalOperator::And,
                torii_proto::LogicalOperator::Or => LogicalOperator::Or,
            }
        }
    }

    impl From<ComparisonOperator> for torii_proto::ComparisonOperator {
        fn from(val: ComparisonOperator) -> Self {
            match val {
                ComparisonOperator::Eq => torii_proto::ComparisonOperator::Eq,
                ComparisonOperator::Neq => torii_proto::ComparisonOperator::Neq,
                ComparisonOperator::Gt => torii_proto::ComparisonOperator::Gt,
                ComparisonOperator::Gte => torii_proto::ComparisonOperator::Gte,
                ComparisonOperator::Lt => torii_proto::ComparisonOperator::Lt,
                ComparisonOperator::Lte => torii_proto::ComparisonOperator::Lte,
                ComparisonOperator::In => torii_proto::ComparisonOperator::In,
                ComparisonOperator::NotIn => torii_proto::ComparisonOperator::NotIn,
                ComparisonOperator::Contains => torii_proto::ComparisonOperator::Contains,
                ComparisonOperator::ContainsAll => torii_proto::ComparisonOperator::ContainsAll,
                ComparisonOperator::ContainsAny => torii_proto::ComparisonOperator::ContainsAny,
                ComparisonOperator::ArrayLengthEq => torii_proto::ComparisonOperator::ArrayLengthEq,
                ComparisonOperator::ArrayLengthGt => torii_proto::ComparisonOperator::ArrayLengthGt,
                ComparisonOperator::ArrayLengthLt => torii_proto::ComparisonOperator::ArrayLengthLt,
            }
        }
    }

    impl From<torii_proto::ComparisonOperator> for ComparisonOperator {
        fn from(val: torii_proto::ComparisonOperator) -> Self {
            match val {
                torii_proto::ComparisonOperator::Eq => ComparisonOperator::Eq,
                torii_proto::ComparisonOperator::Neq => ComparisonOperator::Neq,
                torii_proto::ComparisonOperator::Gt => ComparisonOperator::Gt,
                torii_proto::ComparisonOperator::Gte => ComparisonOperator::Gte,
                torii_proto::ComparisonOperator::Lt => ComparisonOperator::Lt,
                torii_proto::ComparisonOperator::Lte => ComparisonOperator::Lte,
                torii_proto::ComparisonOperator::In => ComparisonOperator::In,
                torii_proto::ComparisonOperator::NotIn => ComparisonOperator::NotIn,
                torii_proto::ComparisonOperator::Contains => ComparisonOperator::Contains,
                torii_proto::ComparisonOperator::ContainsAll => ComparisonOperator::ContainsAll,
                torii_proto::ComparisonOperator::ContainsAny => ComparisonOperator::ContainsAny,
                torii_proto::ComparisonOperator::ArrayLengthEq => ComparisonOperator::ArrayLengthEq,
                torii_proto::ComparisonOperator::ArrayLengthGt => ComparisonOperator::ArrayLengthGt,
                torii_proto::ComparisonOperator::ArrayLengthLt => ComparisonOperator::ArrayLengthLt,
            }
        }
    }

    impl From<CallType> for torii_proto::CallType {
        fn from(val: CallType) -> Self {
            match val {
                CallType::Execute => torii_proto::CallType::Execute,
                CallType::ExecuteFromOutside => torii_proto::CallType::ExecuteFromOutside,
            }
        }
    }

    impl From<torii_proto::CallType> for CallType {
        fn from(val: torii_proto::CallType) -> Self {
            match val {
                torii_proto::CallType::Execute => CallType::Execute,
                torii_proto::CallType::ExecuteFromOutside => CallType::ExecuteFromOutside,
            }
        }
    }
}

