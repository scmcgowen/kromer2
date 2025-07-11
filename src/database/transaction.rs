use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::database::ModelExt;

#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct Model {
    pub id: i32,
    pub amount: Decimal,
    pub from: Option<String>,
    pub to: String,
    pub metadata: Option<String>,
    pub transaction_type: String,
    pub date: DateTime<Utc>,
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

#[async_trait]
impl ModelExt for Model {
    async fn fetch_by_id(pool: &Pool<Postgres>, id: i64) -> sqlx::Result<Option<Self>>
    where
        Self: Sized,
    {
        let q = "SELECT * FROM transaction WHERE id = ?";

        sqlx::query_as(q).bind(id).fetch_optional(pool).await
    }

    async fn fetch_all(pool: &Pool<Postgres>, limit: i64, offset: i64) -> sqlx::Result<Vec<Self>>
    where
        Self: Sized,
    {
        let limit = limit.clamp(0, 1000);
        let q = "SELECT * from transaction LIMIT $1 OFFSET $2";

        sqlx::query_as(q)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await
    }

    async fn total_count(pool: &Pool<Postgres>) -> sqlx::Result<usize> {
        let q = "SELECT COUNT() FROM transaction";
        let result: i64 = sqlx::query_scalar(q).fetch_one(pool).await?;

        Ok(result as usize)
    }
}
