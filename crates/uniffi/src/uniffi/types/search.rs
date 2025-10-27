use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    pub query: String,
    pub limit: u32,
}

impl From<SearchQuery> for torii_proto::SearchQuery {
    fn from(val: SearchQuery) -> Self {
        Self { query: val.query, limit: val.limit }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMatch {
    pub id: String,
    pub fields: HashMap<String, String>,
    pub score: Option<f64>,
}

impl From<torii_proto::SearchMatch> for SearchMatch {
    fn from(val: torii_proto::SearchMatch) -> Self {
        Self { id: val.id, fields: val.fields, score: val.score }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableSearchResults {
    pub table: String,
    pub count: u32,
    pub matches: Vec<SearchMatch>,
}

impl From<torii_proto::TableSearchResults> for TableSearchResults {
    fn from(val: torii_proto::TableSearchResults) -> Self {
        Self {
            table: val.table,
            count: val.count,
            matches: val.matches.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    pub total: u32,
    pub results: Vec<TableSearchResults>,
}

impl From<torii_proto::SearchResponse> for SearchResponse {
    fn from(val: torii_proto::SearchResponse) -> Self {
        Self { total: val.total, results: val.results.into_iter().map(Into::into).collect() }
    }
}
