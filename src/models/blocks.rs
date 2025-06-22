use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct BlockJson {
    pub height: f64,
    pub address: String,
    pub hash: Option<String>,
    pub short_hash: Option<String>,
    pub value: f64,
    pub time: String,
    pub difficulty: f64,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct SubmitBlockResponse {
    pub address: super::addresses::AddressJson,
    pub block: BlockJson,
    pub work: f64,
}
