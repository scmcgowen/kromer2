use async_trait::async_trait;
use sqlx::{Encode, Executor, Postgres, Type};
use uuid::Uuid;

use crate::database::wallet::Model as Wallet;
use crate::database::{DatabaseError, ModelExt, Result};

#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct Model {
    pub id: Uuid,
    pub name: String,
    pub owned_wallets: Vec<i32>,
}

#[async_trait]
impl<'q> ModelExt<'q> for Model {
    async fn fetch_by_id<T, E>(pool: E, id: T) -> Result<Option<Self>>
    where
        Self: Sized,
        T: 'q + Encode<'q, Postgres> + Type<Postgres> + Send,
        E: 'q + Executor<'q, Database = Postgres>,
    {
        let q = "SELECT * FROM players WHERE id = $1";

        sqlx::query_as(q)
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(DatabaseError::Sqlx)
    }

    async fn fetch_all<E>(pool: E, limit: i64, offset: i64) -> Result<Vec<Self>>
    where
        Self: Sized,
        E: 'q + Executor<'q, Database = Postgres>,
    {
        let limit = limit.clamp(1, 1000);
        let q = "SELECT * from players LIMIT $1 OFFSET $2";

        sqlx::query_as(q)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await
            .map_err(DatabaseError::Sqlx)
    }

    async fn total_count<E>(pool: E) -> Result<usize>
    where
        E: 'q + Executor<'q, Database = Postgres>,
    {
        let q = "SELECT COUNT(*) FROM players";
        let result: i64 = sqlx::query_scalar(q).fetch_one(pool).await?;

        Ok(result as usize)
    }
}

impl<'q> Model {
    pub async fn create<E>(executor: E, uuid: Uuid, name: String) -> Result<Model>
    where
        E: 'q + Executor<'q, Database = Postgres>,
    {
        let q = "INSERT INTO players(id, name) VALUES ($1, $2) RETURNING *";

        sqlx::query_as(q)
            .bind(uuid)
            .bind(name)
            .fetch_one(executor)
            .await
            .map_err(DatabaseError::Sqlx)
    }

    pub async fn add_wallet_to_owned<E>(&self, executor: E, wallet: &Wallet) -> Result<Model>
    where
        E: 'q + Executor<'q, Database = Postgres>,
    {
        let q = "UPDATE players SET owned_wallets = array_append(owned_wallets, $2) WHERE id = $1 RETURNING *;";

        sqlx::query_as(q)
            .bind(self.id)
            .bind(wallet.id)
            .fetch_one(executor)
            .await
            .map_err(DatabaseError::Sqlx)
    }

    pub async fn fetch_by_name<E>(pool: E, name: String) -> Result<Option<Self>>
    where
        Self: Sized,
        E: 'q + Executor<'q, Database = Postgres>,
    {
        let q = "SELECT * FROM players WHERE name = $1";

        sqlx::query_as(q)
            .bind(name)
            .fetch_optional(pool)
            .await
            .map_err(DatabaseError::Sqlx)
    }

    /// Get this player's owned wallets.
    pub async fn owned_wallets<E>(&self, executor: E) -> Result<Vec<Wallet>>
    where
        E: 'q + Executor<'q, Database = Postgres>,
    {
        let uuid = &self.id;

        let q = r#"
            SELECT wallet.*
            FROM wallets wallet
            JOIN players player ON wallet.id = ANY(player.owned_wallets)
            WHERE player.id = $1;
            "#;

        sqlx::query_as(q)
            .bind(uuid)
            .fetch_all(executor)
            .await
            .map_err(DatabaseError::Sqlx)
    }
}
