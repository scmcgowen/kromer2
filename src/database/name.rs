use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rust_decimal::{Decimal, dec};
use sqlx::{Acquire, Encode, Executor, Pool, Postgres, Type};

use crate::database::transaction::Model as Transaction;
use crate::database::transaction::{TransactionCreateData, TransactionType};
use crate::database::wallet::Model as Wallet;

use crate::models::websockets::{WebSocketEvent, WebSocketMessage};
use crate::websockets::WebSocketServer;
use crate::{
    database::ModelExt,
    errors::krist::{KristError, address::AddressError, generic::GenericError, name::NameError},
    models::names::NameDataUpdateBody,
    routes::PaginationParams,
    utils::validation,
};

#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct Model {
    pub id: i32,
    pub last_transfered: Option<DateTime<Utc>>,
    pub last_updated: Option<DateTime<Utc>>,
    pub name: String,
    pub owner: String,
    pub original_owner: String,
    pub time_registered: DateTime<Utc>,
    pub unpaid: Decimal,
    pub metadata: Option<String>,
}

#[async_trait]
impl<'q> ModelExt<'q> for Model {
    async fn fetch_by_id<T, E>(pool: E, id: T) -> sqlx::Result<Option<Self>>
    where
        Self: Sized,
        T: 'q + Encode<'q, Postgres> + Type<Postgres> + Send,
        E: 'q + Executor<'q, Database = Postgres>,
    {
        let q = "SELECT * FROM names WHERE id = $1";

        sqlx::query_as(q).bind(id).fetch_optional(pool).await
    }

    async fn fetch_all<E>(pool: E, limit: i64, offset: i64) -> sqlx::Result<Vec<Self>>
    where
        Self: Sized,
        E: 'q + Executor<'q, Database = Postgres>,
    {
        let limit = limit.clamp(1, 1000);
        let q = "SELECT * from names LIMIT $1 OFFSET $2";

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
        let q = "SELECT COUNT(*) FROM names";
        let result: i64 = sqlx::query_scalar(q).fetch_one(pool).await?;

        Ok(result as usize)
    }
}

impl<'q> Model {
    /// Get name from its name field
    pub async fn fetch_by_name<S, E>(pool: E, name: S) -> sqlx::Result<Option<Model>>
    where
        S: AsRef<str>,
        E: 'q + Executor<'q, Database = Postgres>,
    {
        let name = name.as_ref();
        let q = "SELECT * FROM names WHERE name = $1;";

        sqlx::query_as(q).bind(name).fetch_optional(pool).await
    }

    pub async fn all_unpaid<E>(pool: E, pagination: &PaginationParams) -> sqlx::Result<Vec<Model>>
    where
        E: 'q + Executor<'q, Database = Postgres>,
    {
        let limit = pagination.limit.unwrap_or(50);
        let offset = pagination.offset.unwrap_or(0);
        let limit = limit.clamp(1, 1000);

        let q = "SELECT * FROM names WHERE unpaid > 0 LIMIT $1 OFFSET $2";

        sqlx::query_as(q)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await
    }

    pub async fn count_unpaid<E>(pool: E) -> sqlx::Result<i64>
    where
        E: 'q + Executor<'q, Database = Postgres>,
    {
        let q = "SELECT count(*) FROM names WHERE unpaid > 0";

        sqlx::query_scalar(q).fetch_one(pool).await
    }

    pub async fn create<E>(pool: E, name: String, owner: String) -> sqlx::Result<Model>
    where
        E: 'q + Executor<'q, Database = Postgres>,
    {
        let q = "INSERT INTO names(name, owner, original_owner, time_registered) VALUES ($1, $2, $2, NOW()) RETURNING *";

        sqlx::query_as(q)
            .bind(name)
            .bind(owner)
            .fetch_one(pool)
            .await
    }

    pub async fn update_metadata<E>(pool: E, name: String, metadata: String) -> sqlx::Result<Model>
    where
        E: 'q + Executor<'q, Database = Postgres>,
    {
        let q = "UPDATE names SET metadata = $2 WHERE name = $1 RETURNING *";

        sqlx::query_as(q)
            .bind(name)
            .bind(metadata)
            .fetch_one(pool)
            .await
    }

    pub async fn ctrl_update_metadata(
        pool: &Pool<Postgres>,
        name: String,
        body: NameDataUpdateBody,
    ) -> Result<Model, KristError> {
        let metadata_record = match body.a {
            Some(metadata_record) => metadata_record,
            None => {
                return Err(KristError::Generic(GenericError::InvalidParameter(
                    "name".to_owned(),
                )));
            }
        };

        if !validation::is_valid_name(&name, false) {
            return Err(KristError::Generic(GenericError::InvalidParameter(
                "name".to_owned(),
            )));
        }

        if !validation::is_valid_a_record(&metadata_record) {
            return Err(KristError::Generic(GenericError::InvalidParameter(
                "a".to_owned(),
            )));
        }

        let name = name.trim().to_lowercase();
        let wallet = Wallet::verify_address(pool, body.private_key).await?;
        if !wallet.authed {
            tracing::info!("Auth failed on name update");
            return Err(KristError::Address(AddressError::AuthFailed));
        }

        let model = Model::fetch_by_name(pool, &name)
            .await?
            .ok_or_else(|| KristError::Name(NameError::NameNotFound(name.clone())))?;
        if model.owner != wallet.model.address {
            return Err(KristError::Name(NameError::NotNameOwner(name)));
        }

        if model.metadata == Some(metadata_record.clone()) {
            return Ok(model);
        }

        let updated_model = Self::update_metadata(pool, name, metadata_record).await?;

        Ok(updated_model)
    }

    /// Fetches the owner of the wallet and returns its database model.
    pub async fn owner<A>(&self, conn: A) -> sqlx::Result<Option<Wallet>>
    where
        A: Acquire<'q, Database = Postgres>,
    {
        let mut tx = conn.begin().await?;

        let owner = Wallet::fetch_by_address(&mut *tx, &self.owner).await?;

        tx.commit().await?;

        Ok(owner)
    }

    /// Transfer ownership to a new wallet
    pub async fn transfer_ownership<A>(
        self,
        conn: A,
        server: &WebSocketServer,
        new_owner_address: String,
    ) -> sqlx::Result<Model>
    where
        A: Acquire<'q, Database = Postgres>,
    {
        let mut tx = conn.begin().await?;
        let q = "UPDATE names SET owner = $2, last_updated = NOW(), last_transfered = NOW() WHERE owner = $1 RETURNING *";

        let updated_name: Model = sqlx::query_as(q)
            .bind(&self.owner)
            .bind(&new_owner_address)
            .fetch_one(&mut *tx)
            .await?;

        let creation_data = TransactionCreateData {
            from: self.owner,
            to: new_owner_address,
            amount: dec!(0),
            metadata: None,
            name: Some(self.name),
            sent_metaname: None,
            sent_name: None,
            transaction_type: TransactionType::NameTransfer,
        };

        let transaction = Transaction::create(&mut *tx, creation_data).await?;
        let event = WebSocketMessage::new_event(WebSocketEvent::Transaction {
            transaction: transaction.into(),
        });
        server.broadcast_event(event).await;

        tx.commit().await?;

        Ok(updated_name)
    }
}
