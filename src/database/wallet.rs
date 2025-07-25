use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rust_decimal::{Decimal, dec};
use sqlx::{Acquire, Encode, Executor, Pool, Postgres, Type};

use crate::database::{ModelExt, name, transaction};
use crate::errors::KromerError;
use crate::errors::wallet::WalletError;
use crate::models::transactions::AddressTransactionQuery;
use crate::routes::PaginationParams;
use crate::utils::crypto;

#[derive(Debug, Clone, PartialEq, sqlx::FromRow, serde::Serialize)]
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
impl<'q> ModelExt<'q> for Model {
    async fn fetch_by_id<T, E>(pool: E, id: T) -> sqlx::Result<Option<Self>>
    where
        Self: Sized,
        T: 'q + Encode<'q, Postgres> + Type<Postgres> + Send,
        E: 'q + Executor<'q, Database = Postgres>,
    {
        let q = "SELECT * FROM wallets WHERE id = $1";

        sqlx::query_as(q).bind(id).fetch_optional(pool).await
    }

    async fn fetch_all<E>(pool: E, limit: i64, offset: i64) -> sqlx::Result<Vec<Self>>
    where
        Self: Sized,
        E: 'q + Executor<'q, Database = Postgres>,
    {
        let limit = limit.clamp(1, 1000);
        let q = "SELECT * from wallets LIMIT $1 OFFSET $2";

        sqlx::query_as(q)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await
    }

    async fn total_count<E>(pool: E) -> sqlx::Result<usize>
    where
        E: 'q + Executor<'q, Database = Postgres>,
    {
        let q = "SELECT COUNT(*) FROM wallets";
        let result: i64 = sqlx::query_scalar(q).fetch_one(pool).await?;

        Ok(result as usize)
    }
}

impl<'q> Model {
    pub async fn fetch_by_address<S, E>(pool: E, address: S) -> sqlx::Result<Option<Self>>
    where
        S: AsRef<str>,
        E: 'q + Executor<'q, Database = Postgres>,
    {
        let address = address.as_ref();

        let q = "SELECT * FROM wallets WHERE address = $1;";
        sqlx::query_as(q).bind(address).fetch_optional(pool).await
    }

    pub async fn fetch_richest<E>(pool: E, limit: i64, offset: i64) -> sqlx::Result<Vec<Self>>
    where
        E: 'q + Executor<'q, Database = Postgres>,
    {
        let limit = limit.clamp(1, 1000);

        let q = "SELECT * FROM wallets ORDER BY balance DESC LIMIT $1 OFFSET $2;";
        sqlx::query_as(q)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await
    }

    #[tracing::instrument(skip(pool))]
    pub async fn verify_address<S>(
        pool: &Pool<Postgres>,
        private_key: S,
    ) -> sqlx::Result<VerifyResponse>
    where
        S: AsRef<str> + std::fmt::Debug,
    {
        let private_key = private_key.as_ref();
        let mut tx = pool.acquire().await?;

        let address = crypto::make_v2_address(private_key, "k");
        let guh = format!("{address}{private_key}");

        tracing::info!("Authentication attempt on address {address}");

        let result = Model::fetch_by_address(&mut *tx, &address).await?;
        let hash = crypto::sha256(&guh);

        let wallet = match result {
            Some(w) => w,
            None => Self::create_wallet(&mut *tx, &address, &hash, None).await?,
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

    pub async fn create_wallet<E>(
        pool: E,
        address: &str,
        hash: &str,
        initial_balance: Option<Decimal>,
    ) -> sqlx::Result<Model>
    where
        E: 'q + Executor<'q, Database = Postgres>,
    {
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

    pub async fn transactions<S, E>(
        pool: E,
        address: S,
        query: &AddressTransactionQuery,
    ) -> sqlx::Result<Vec<transaction::Model>>
    where
        S: AsRef<str>,
        E: 'q + Executor<'q, Database = Postgres>,
    {
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

    pub async fn names<S, E>(
        pool: E,
        address: S,
        query: &PaginationParams,
    ) -> sqlx::Result<Vec<name::Model>>
    where
        S: AsRef<str>,
        E: 'q + Executor<'q, Database = Postgres>,
    {
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

    pub async fn set_balance<S>(
        executor: &Pool<Postgres>,
        address: S,
        balance: Decimal,
    ) -> Result<Model, KromerError>
    where
        S: AsRef<str>,
    {
        let address = address.as_ref();

        // Just make sure that wallet exists
        // TODO: Make a generic function on ModelExt trait that returns whether or not something exists.
        let _wallet = Self::fetch_by_address(executor, address)
            .await?
            .ok_or_else(|| KromerError::Wallet(WalletError::NotFound))?;

        let q = "UPDATE wallets SET balance = $1 WHERE address = $2 RETURNING *";

        sqlx::query_as(q)
            .bind(balance)
            .bind(address)
            .fetch_one(executor)
            .await
            .map_err(KromerError::Database)
    }

    #[tracing::instrument(skip(conn))]
    pub async fn update_balance<S, A>(
        conn: A,
        address: S,
        balance: Decimal,
    ) -> Result<Model, KromerError>
    where
        S: AsRef<str> + std::fmt::Debug,
        A: Acquire<'q, Database = Postgres>,
    {
        let mut tx = conn.begin().await?;
        let address = address.as_ref();

        // Just make sure that wallet exists
        // TODO: Make a generic function on ModelExt trait that returns whether or not something exists.
        let wallet_exists: bool =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM wallets WHERE address = $1)")
                .bind(address)
                .fetch_one(&mut *tx)
                .await?;
        if !wallet_exists {
            tracing::debug!("");
            return Err(KromerError::Wallet(WalletError::NotFound));
        }

        let q = r#"
        UPDATE wallets
        SET
            balance = balance + $1,
            total_in = total_in + CASE WHEN $1 > 0 THEN $1 ELSE 0 END,
            total_out = abs(total_out) + CASE WHEN $1 < 0 THEN abs($1) ELSE 0 END
        WHERE address = $2
        RETURNING *;
        "#;

        let model = sqlx::query_as(q)
            .bind(balance)
            .bind(address)
            .fetch_one(&mut *tx)
            .await
            .map_err(KromerError::Database)?;
        tx.commit().await?;

        Ok(model)
    }
}
