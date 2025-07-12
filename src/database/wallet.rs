use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rust_decimal::{Decimal, dec};
use sqlx::{Pool, Postgres};

use crate::database::{ModelExt, name, transaction};
use crate::models::transactions::AddressTransactionQuery;
use crate::routes::PaginationParams;
use crate::utils::crypto;

#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct Model {
    pub id: i32,
    pub address: String,
    pub balance: Decimal,
    pub created_at: DateTime<Utc>,
    pub locked: bool,
    pub total_in: Decimal,
    pub total_out: Decimal,
    pub private_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VerifyResponse {
    pub authed: bool,
    pub model: Model,
}

#[async_trait]
impl ModelExt for Model {
    async fn fetch_by_id(pool: &Pool<Postgres>, id: i64) -> sqlx::Result<Option<Self>>
    where
        Self: Sized,
    {
        let q = "SELECT * FROM wallets WHERE id = $1";

        sqlx::query_as(q).bind(id).fetch_optional(pool).await
    }

    async fn fetch_all(pool: &Pool<Postgres>, limit: i64, offset: i64) -> sqlx::Result<Vec<Self>>
    where
        Self: Sized,
    {
        let limit = limit.clamp(1, 1000);

        let q = "SELECT * from wallets LIMIT $1 OFFSET $2";

        sqlx::query_as(q)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await
    }

    async fn total_count(pool: &Pool<Postgres>) -> sqlx::Result<usize> {
        let q = "SELECT COUNT(*) FROM wallets;";
        let result: i64 = sqlx::query_scalar(q).fetch_one(pool).await?;

        Ok(result as usize)
    }
}

impl Model {
    pub async fn fetch_by_address<S: AsRef<str>>(
        pool: &Pool<Postgres>,
        address: S,
    ) -> sqlx::Result<Option<Self>> {
        let address = address.as_ref();

        let q = "SELECT * FROM wallets WHERE address = $1;";
        sqlx::query_as(q).bind(address).fetch_optional(pool).await
    }

    pub async fn fetch_richest(
        pool: &Pool<Postgres>,
        limit: i64,
        offset: i64,
    ) -> sqlx::Result<Vec<Self>> {
        let limit = limit.clamp(1, 1000);

        let q = "SELECT * FROM wallets ORDER BY balance DESC LIMIT $1 OFFSET $2;";
        sqlx::query_as(q)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await
    }

    #[tracing::instrument(skip(pool))]
    pub async fn verify_address<S: AsRef<str> + std::fmt::Debug>(
        pool: &Pool<Postgres>,
        private_key: S,
    ) -> sqlx::Result<VerifyResponse> {
        let private_key = private_key.as_ref();

        let address = crypto::make_v2_address(private_key, "k");
        let guh = format!("{address}{private_key}");

        tracing::info!("Authentication attempt on address {address}");

        let result = Model::fetch_by_address(pool, &address).await?;
        let hash = crypto::sha256(&guh);

        let wallet = match result {
            Some(w) => w,
            None => Self::create_wallet(pool, &address, &hash, None).await?,
        };
        let pkey = &wallet.private_key;

        let authed = *pkey == Some(hash);
        if !authed {
            tracing::info!("Someone tried to login to an address they do not own");
        }

        return Ok(VerifyResponse {
            authed,
            model: wallet,
        });
    }

    pub async fn create_wallet(
        pool: &Pool<Postgres>,
        address: &str,
        hash: &str,
        initial_balance: Option<Decimal>,
    ) -> sqlx::Result<Model> {
        let initial_balance = initial_balance.unwrap_or(dec!(0.0));

        // Pretty big query, lol
        let q = "INSERT INTO wallets(address, balance, created_at, private_key) VALUES ($1, $2, NOW(), $3) RETURNING *";

        sqlx::query_as(q)
            .bind(address)
            .bind(initial_balance)
            .bind(hash)
            .fetch_one(pool)
            .await
    }

    pub async fn transactions<S: AsRef<str>>(
        pool: &Pool<Postgres>,
        address: S,
        query: &AddressTransactionQuery,
    ) -> sqlx::Result<Vec<transaction::Model>> {
        let address = address.as_ref().to_owned();

        let limit = query.limit.unwrap_or(50);
        let offset = query.offset.unwrap_or(0);
        let limit = limit.clamp(1, 1000);

        let q = r#"
            SELECT * FROM transactions
            WHERE "from" = $1 OR "to" = $1
            ORDER BY date DESC
            LIMIT $2 OFFSET $3;
        "#;
        sqlx::query_as(q)
            .bind(address)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await
    }

    pub async fn names<S: AsRef<str>>(
        pool: &Pool<Postgres>,
        address: S,
        query: &PaginationParams,
    ) -> sqlx::Result<Vec<name::Model>> {
        let address = address.as_ref().to_owned();

        let limit = query.limit.unwrap_or(50);
        let offset = query.offset.unwrap_or(0);
        let limit = limit.clamp(1, 1000);

        let q = r#"SELECT * FROM names WHERE owner = $1 ORDER BY name ASC LIMIT $2 OFFSET $3;"#;
        sqlx::query_as(q)
            .bind(address)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await
    }
}
