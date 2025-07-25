use async_trait::async_trait;
use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use regex::Regex;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{Encode, Executor, Pool, Postgres, Type};

use crate::{database::ModelExt, routes::PaginationParams};

use crate::database::wallet::Model as Wallet;
use crate::errors::{KromerError, wallet::WalletError};

static KRO_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(?:([a-z0-9-_]{1,32})@)?([a-z0-9]{1,64})\.kro").unwrap());

#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct Model {
    pub id: i32,
    pub amount: Decimal,
    pub from: Option<String>,
    pub to: String,
    pub metadata: Option<String>,
    pub name: Option<String>,
    pub sent_metaname: Option<String>,
    pub sent_name: Option<String>,
    pub transaction_type: TransactionType,
    pub date: DateTime<Utc>,
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "transaction_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum TransactionType {
    #[default]
    Mined,
    Unknown,
    NamePurchase,
    NameARecord,
    NameTransfer,
    Transfer,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct TransactionCreateData {
    pub from: String,
    pub to: String,
    pub amount: Decimal,
    pub metadata: Option<String>,
    pub name: Option<String>,
    pub sent_metaname: Option<String>,
    pub sent_name: Option<String>,
    pub transaction_type: TransactionType,
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct TransactionNameData {
    pub name: Option<String>,
    pub metaname: Option<String>,
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
impl<'q> ModelExt<'q> for Model {
    async fn fetch_by_id<T, E>(pool: E, id: T) -> sqlx::Result<Option<Self>>
    where
        Self: Sized,
        T: 'q + Encode<'q, Postgres> + Type<Postgres> + Send,
        E: 'q + Executor<'q, Database = Postgres>,
    {
        let q = "SELECT * FROM transactions WHERE id = $1";

        sqlx::query_as(q).bind(id).fetch_optional(pool).await
    }

    async fn fetch_all<E>(pool: E, limit: i64, offset: i64) -> sqlx::Result<Vec<Self>>
    where
        Self: Sized,
        E: 'q + Executor<'q, Database = Postgres>,
    {
        let limit = limit.clamp(1, 1000);
        let q = "SELECT * from transactions LIMIT $1 OFFSET $2";

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
        let q = "SELECT COUNT(*) FROM transactions";
        let result: i64 = sqlx::query_scalar(q).fetch_one(pool).await?;

        Ok(result as usize)
    }
}

impl Model {
    pub async fn sorted_by_date(
        pool: &Pool<Postgres>,
        pagination: &PaginationParams,
    ) -> sqlx::Result<Vec<Model>> {
        let limit = pagination.limit.unwrap_or(50);
        let offset = pagination.offset.unwrap_or(0);
        let limit = limit.clamp(1, 1000);

        let q = r#"SELECT * FROM transactions ORDER BY date DESC LIMIT $1 OFFSET $2;"#;

        sqlx::query_as(q)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await
    }

    pub async fn create_no_update(
        pool: &Pool<Postgres>,
        creation_data: TransactionCreateData,
    ) -> sqlx::Result<Model> {
        let metadata = creation_data.metadata.unwrap_or_default();

        let mut tx = pool.begin().await?;

        let q = r#"INSERT INTO transactions(amount, "from", "to", metadata, transaction_type, date) VALUES ($1, $2, $3, $4, $5, NOW()) RETURNING *"#;

        let model = sqlx::query_as(q)
            .bind(creation_data.amount)
            .bind(&creation_data.from)
            .bind(&creation_data.to)
            .bind(metadata)
            .bind(creation_data.transaction_type)
            .fetch_one(&mut *tx)
            .await?;
        tx.commit().await?;

        Ok(model)
    }

    pub async fn create(
        pool: &Pool<Postgres>,
        creation_data: TransactionCreateData,
    ) -> sqlx::Result<Model> {
        let metadata = creation_data.metadata.unwrap_or_default();

        let mut tx = pool.begin().await?;

        let q = r#"INSERT INTO transactions(amount, "from", "to", metadata, transaction_type, date, name, sent_metaname, sent_name) VALUES ($1, $2, $3, $4, $5, NOW(), $6, $7, $8) RETURNING *"#;

        // TODO: Factor this out to the db, thank god its a transaction now.
        let _ = Wallet::update_balance(&mut *tx, &creation_data.from, -creation_data.amount)
            .await
            .map_err(|_| KromerError::Wallet(WalletError::NotFound));

        let _ = Wallet::update_balance(&mut *tx, &creation_data.to, creation_data.amount)
            .await
            .map_err(|_| KromerError::Wallet(WalletError::NotFound));

        let model = sqlx::query_as(q)
            .bind(creation_data.amount)
            .bind(&creation_data.from)
            .bind(&creation_data.to)
            .bind(metadata)
            .bind(creation_data.transaction_type)
            .bind(creation_data.name)
            .bind(creation_data.sent_metaname)
            .bind(creation_data.sent_name)
            .fetch_one(&mut *tx)
            .await?;
        tx.commit().await?; // I'm not sure this is how it should be done? `Wallet::update_balance` also creates a transaction..

        Ok(model)
    }
}

impl TransactionNameData {
    /// Parse a transaction name from a string-like type according to CommonMeta format.
    /// Takes any type that can be converted to a string reference.
    ///
    /// If the input is empty, returns a default `TransactionNameData`.
    /// Otherwise parses according to the pattern: `meta@name.kro`
    ///
    /// # Examples
    /// ```
    /// use kromer::database::transaction::TransactionNameData;
    /// let data = TransactionNameData::parse("meta@name.kro");
    /// assert_eq!(data.metaname, Some("meta".to_string()));
    /// assert_eq!(data.name, Some("name".to_string()));
    ///
    /// let empty = TransactionNameData::parse("");
    /// assert_eq!(empty, TransactionNameData::default());
    /// ```
    pub fn parse<S: AsRef<str>>(input: S) -> Self {
        let input = input.as_ref();
        if input.is_empty() {
            return Self::default(); // Don't do useless parsing if the input is empty, thats silly.
        }

        match KRO_REGEX.captures(input) {
            Some(captures) => {
                let metaname = captures.get(1).map(|m| m.as_str().to_string()); // TODO: Less allocating, should maybe use `&str` on the transaction models
                let name = captures.get(2).map(|m| m.as_str().to_string());

                Self { metaname, name }
            }
            None => Self::default(),
        }
    }

    /// Parse a transaction name from an optional string-like type.
    /// If the input is `None`, returns a default `TransactionNameData`.
    /// Otherwise, parses the string according to CommonMeta format.
    ///
    /// # Examples
    /// ```
    /// use kromer::database::transaction::TransactionNameData;
    /// let data = TransactionNameData::parse_opt(Some("meta@name.kro"));
    /// assert_eq!(data.metaname, Some("meta".to_string()));
    /// assert_eq!(data.name, Some("name".to_string()));
    ///
    /// let empty = TransactionNameData::parse_opt(None::<String>);
    /// assert_eq!(empty, TransactionNameData::default());
    /// ```
    pub fn parse_opt<S: AsRef<str>>(input: Option<S>) -> Self {
        if input.is_none() {
            return Self::default(); // Do we really need to parse stuff is there is no value? No, we dont.
        }

        let input = input.unwrap(); // We can do this, we made sure it exists.
        Self::parse(input)
    }

    /// Parse a transaction name from a reference to an optional string-like type.
    /// If the input is `None`, returns a default `TransactionNameData`.
    /// Otherwise, parses the string according to CommonMeta format.
    ///
    /// # Examples
    /// ```
    /// use kromer::database::transaction::TransactionNameData;
    /// let input = Some("meta@name.kro".to_string());
    /// let data = TransactionNameData::parse_opt_ref(&input);
    /// assert_eq!(data.metaname, Some("meta".to_string()));
    /// assert_eq!(data.name, Some("name".to_string()));
    ///
    /// let empty = TransactionNameData::parse_opt_ref(&None::<String>);
    /// assert_eq!(empty, TransactionNameData::default());
    /// ```
    pub fn parse_opt_ref<S: AsRef<str>>(input: &Option<S>) -> Self {
        if input.is_none() {
            return Self::default(); // Do we really need to parse stuff is there is no value? No, we dont.
        }

        let input = input.as_ref().unwrap(); // We can do this, we made sure it exists.
        Self::parse(input)
    }

    /// Return the name as a string slice.
    #[inline(always)]
    pub fn name(&self) -> Option<&str> {
        let name_ref = self.name.as_ref();

        name_ref.map(|name| name.as_str())
    }

    /// Return the metaname as a string slice
    #[inline(always)]
    pub fn metaname(&self) -> Option<&str> {
        let metaname_ref = self.metaname.as_ref();

        metaname_ref.map(|metaname| metaname.as_str())
    }
}
