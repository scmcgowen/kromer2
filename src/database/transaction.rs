use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sea_query::Iden;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
pub struct Model {
    pub id: i32,
    pub amount: Decimal,
    pub from: Option<String>,
    pub to: String,
    pub metadata: Option<String>,
    pub transaction_type: String,
    pub date: DateTime<Utc>,
}

#[derive(Debug, Iden)]
pub enum Player {
    Table,
    Amount,
    From,
    To,
    Metadata,
    TransactionType,
    Date,
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransactionType {
    #[default]
    Unknown,
    Mined,
    NamePurchase,
    NameARecord,
    NameTransfer,
    Transfer,
}

impl From<String> for TransactionType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "mined" => TransactionType::Mined,
            "name_purchase" => TransactionType::NamePurchase,
            "name_a_record" => TransactionType::NameARecord,
            "name_transfer" => TransactionType::NameTransfer,
            "transfer" => TransactionType::Transfer,
            _ => TransactionType::Unknown,
        }
    }
}

impl From<TransactionType> for &str {
    fn from(value: TransactionType) -> Self {
        match value {
            TransactionType::Unknown => "unknown",
            TransactionType::Mined => "mined",
            TransactionType::NamePurchase => "name_purchase",
            TransactionType::NameARecord => "name_a_record",
            TransactionType::NameTransfer => "name_transfer",
            TransactionType::Transfer => "transfer",
        }
    }
}
