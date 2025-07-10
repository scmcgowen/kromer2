use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::{MySql, Pool};

use crate::database::ModelExt;

#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct Model {
    pub id: i64,
    pub address: String,
    pub balance: Decimal,
    pub created_at: DateTime<Utc>,
    pub locked: bool,
    pub total_in: Decimal,
    pub total_out: Decimal,
}

#[async_trait]
impl ModelExt for Model {
    async fn fetch_by_id(pool: &Pool<MySql>, id: i64) -> sqlx::Result<Option<Self>>
    where
        Self: Sized,
    {
        let q = "SELECT * FROM wallets WHERE id = ?";

        sqlx::query_as(q).bind(id).fetch_optional(pool).await
    }

    async fn fetch_all(pool: &Pool<MySql>, limit: u64, offset: u64) -> sqlx::Result<Vec<Self>>
    where
        Self: Sized,
    {
        let limit = limit.clamp(0, 1000);

        let q = "SELECT * from wallets LIMIT ? OFFSET ?";

        sqlx::query_as(q)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await
    }

    async fn total_count(pool: &Pool<MySql>) -> sqlx::Result<usize> {
        let q = "SELECT COUNT(*) FROM wallets;";
        let result: i64 = sqlx::query_scalar(q).fetch_one(pool).await?;

        Ok(result as usize)
    }
}

impl Model {
    pub async fn fetch_by_address<S: AsRef<str>>(
        pool: &Pool<MySql>,
        address: S,
    ) -> sqlx::Result<Option<Self>> {
        let address = address.as_ref();

        let q = "SELECT * FROM wallets WHERE address = ?;";
        sqlx::query_as(q).bind(address).fetch_optional(pool).await
    }

    pub async fn fetch_richest(
        pool: &Pool<MySql>,
        limit: u64,
        offset: u64,
    ) -> sqlx::Result<Vec<Self>> {
        let limit = limit.clamp(0, 1000);

        let q = "SELECT * FROM wallets ORDER BY balance DESC LIMIT ? OFFSET ?;";
        sqlx::query_as(q)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await
    }
}
