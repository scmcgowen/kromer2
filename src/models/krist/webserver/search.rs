use crate::models::krist::addresses::AddressJson;
use crate::models::krist::blocks::BlockJson;
use crate::models::krist::names::NameJson;
use crate::models::krist::transactions::TransactionJson;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReqSearchQuery {
    pub limit: Option<String>,
    pub offset: Option<String>,
    pub order_by: Option<String>,
    pub order: Option<String>,
    pub q: Option<String>,
    pub include_mined: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchQueryMatch {
    pub original_query: String,
    pub match_address: bool,
    pub match_block: bool,
    pub match_name: bool,
    pub match_transaction: bool,
    pub stripped_name: String,
    #[serde(rename = "hasID")]
    pub has_id: bool,
    #[serde(rename = "cleanID")]
    pub clean_id: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub query: SearchQueryMatch,
    pub matches: SearchResultMatches,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultMatches {
    pub exact_address: Option<AddressJson>,
    pub exact_block: Option<BlockJson>,
    pub exact_name: Option<NameJson>,
    pub exact_transaction: Option<TransactionJson>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchExtendedResult {
    pub query: SearchResult,
    pub matches: SearchExtendedResultMatches,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchExtendedResultMatches {
    pub transactions: SearchExtendedResultTransactions,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchExtendedResultTransactions {
    pub address_involved: Option<f64>,
    pub name_involved: Option<f64>,
    pub metadata: Option<f64>,
}
