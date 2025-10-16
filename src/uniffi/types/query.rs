// Query types - Query, Clause, KeysClause, MemberClause, CompositeClause
use super::core::*;
use super::schema::MemberValue;

// SQL query result types
#[derive(Debug, Clone)]
pub struct SqlField {
    pub name: String,
    pub value: SqlValue,
}

#[derive(Debug, Clone)]
pub struct SqlRow {
    pub fields: Vec<SqlField>,
}

#[derive(Debug, Clone)]
pub enum SqlValue {
    Text { value: String },
    Integer { value: i64 },
    Real { value: f64 },
    Blob { value: Vec<u8> },
    Null,
}

impl TryInto<SqlRow> for torii_proto::SqlRow {
    type Error = DojoError;
    
    fn try_into(self) -> Result<SqlRow, Self::Error> {
        let fields = self.fields.into_iter().map(|(name, v)| {
            let value = match v {
                torii_proto::SqlValue::Text(s) => SqlValue::Text { value: s },
                torii_proto::SqlValue::Integer(i) => SqlValue::Integer { value: i },
                torii_proto::SqlValue::Real(r) => SqlValue::Real { value: r },
                torii_proto::SqlValue::Blob(b) => SqlValue::Blob { value: b },
                torii_proto::SqlValue::Null => SqlValue::Null,
            };
            SqlField { name, value }
        }).collect();
        Ok(SqlRow { fields })
    }
}

#[derive(Debug, Clone)]
pub enum PatternMatching {
    FixedLen,
    VariableLen,
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

#[derive(Debug, Clone)]
pub struct KeysClause {
    pub keys: Vec<Option<FieldElement>>,
    pub pattern_matching: PatternMatching,
    pub models: Vec<String>,
}

impl From<KeysClause> for torii_proto::KeysClause {
    fn from(val: KeysClause) -> Self {
        torii_proto::KeysClause {
            keys: val
                .keys
                .into_iter()
                .map(|k| k.map(|s| field_element_to_felt(&s).unwrap()))
                .collect(),
            pattern_matching: val.pattern_matching.into(),
            models: val.models,
        }
    }
}

impl From<torii_proto::KeysClause> for KeysClause {
    fn from(val: torii_proto::KeysClause) -> Self {
        KeysClause {
            models: val.models,
            keys: val.keys.into_iter().map(|k| k.map(felt_to_field_element)).collect(),
            pattern_matching: val.pattern_matching.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum LogicalOperator {
    And,
    Or,
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct MemberClause {
    pub model: String,
    pub member: String,
    pub operator: ComparisonOperator,
    pub value: MemberValue,
}

impl From<MemberClause> for torii_proto::MemberClause {
    fn from(val: MemberClause) -> Self {
        torii_proto::MemberClause {
            member: val.member,
            model: val.model,
            operator: val.operator.into(),
            value: val.value.into(),
        }
    }
}

impl From<torii_proto::MemberClause> for MemberClause {
    fn from(val: torii_proto::MemberClause) -> Self {
        MemberClause {
            model: val.model,
            member: val.member,
            operator: val.operator.into(),
            value: val.value.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CompositeClause {
    pub operator: LogicalOperator,
    pub clauses: Vec<Clause>,
}

impl From<CompositeClause> for torii_proto::CompositeClause {
    fn from(val: CompositeClause) -> Self {
        torii_proto::CompositeClause {
            operator: val.operator.into(),
            clauses: val.clauses.into_iter().map(|c| c.into()).collect(),
        }
    }
}

impl From<torii_proto::CompositeClause> for CompositeClause {
    fn from(val: torii_proto::CompositeClause) -> Self {
        CompositeClause {
            operator: val.operator.into(),
            clauses: val.clauses.into_iter().map(|c| c.into()).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Clause {
    HashedKeys { keys: Vec<FieldElement> },
    Keys { clause: KeysClause },
    Member { clause: MemberClause },
    Composite { clause: CompositeClause },
}

impl From<Clause> for torii_proto::Clause {
    fn from(val: Clause) -> Self {
        match val {
            Clause::HashedKeys { keys } => torii_proto::Clause::HashedKeys(
                keys.into_iter().map(|k| field_element_to_felt(&k).unwrap()).collect(),
            ),
            Clause::Keys { clause } => torii_proto::Clause::Keys(clause.into()),
            Clause::Member { clause } => torii_proto::Clause::Member(clause.into()),
            Clause::Composite { clause } => torii_proto::Clause::Composite(clause.into()),
        }
    }
}

impl From<torii_proto::Clause> for Clause {
    fn from(val: torii_proto::Clause) -> Self {
        match val {
            torii_proto::Clause::HashedKeys(keys) => {
                Clause::HashedKeys { keys: keys.into_iter().map(felt_to_field_element).collect() }
            }
            torii_proto::Clause::Keys(clause) => Clause::Keys { clause: clause.into() },
            torii_proto::Clause::Member(clause) => Clause::Member { clause: clause.into() },
            torii_proto::Clause::Composite(clause) => Clause::Composite { clause: clause.into() },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Query {
    pub world_addresses: Vec<FieldElement>,
    pub pagination: Pagination,
    pub clause: Option<Clause>,
    pub no_hashed_keys: bool,
    pub models: Vec<String>,
    pub historical: bool,
}

impl From<Query> for torii_proto::Query {
    fn from(val: Query) -> Self {
        torii_proto::Query {
            world_addresses: val
                .world_addresses
                .into_iter()
                .map(|a| field_element_to_felt(&a).unwrap())
                .collect(),
            pagination: val.pagination.into(),
            clause: val.clause.map(|c| c.into()),
            models: val.models,
            no_hashed_keys: val.no_hashed_keys,
            historical: val.historical,
        }
    }
}

impl From<torii_proto::Query> for Query {
    fn from(val: torii_proto::Query) -> Self {
        Query {
            world_addresses: val.world_addresses.into_iter().map(felt_to_field_element).collect(),
            pagination: val.pagination.into(),
            clause: val.clause.map(|c| c.into()),
            models: val.models,
            no_hashed_keys: val.no_hashed_keys,
            historical: val.historical,
        }
    }
}

