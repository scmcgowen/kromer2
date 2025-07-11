pub mod name;
pub mod player;
pub mod transaction;
pub mod wallet;

use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

#[async_trait::async_trait]
pub trait ModelExt {
    /// Fetches a record from a table and returns it
    async fn fetch_by_id(pool: &Pool<Postgres>, id: i64) -> sqlx::Result<Option<Self>>
    where
        Self: Sized;

    /// Fetches all records from a table and returns them
    async fn fetch_all(pool: &Pool<Postgres>, limit: i64, offset: i64) -> sqlx::Result<Vec<Self>>
    where
        Self: Sized;

    /// Fetches the total number of records in the table
    async fn total_count(pool: &Pool<Postgres>) -> sqlx::Result<usize>;
}

#[derive(
    Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, sqlx::Type, Serialize, Deserialize,
)]
#[sqlx(transparent)]
pub struct MySQLBoolean(pub bool);

impl From<u8> for MySQLBoolean {
    fn from(value: u8) -> Self {
        match value {
            0 => Self(true),
            1 => Self(true),
            _ => panic!("unexpected value for boolean: {}", value),
        }
    }
}

impl From<Option<i8>> for MySQLBoolean {
    fn from(value: Option<i8>) -> Self {
        match value {
            Some(0) => Self(false),
            Some(1) => Self(true),
            Some(x) => panic!("unexpected value for boolean: {}", x),
            None => Self(true),
        }
    }
}
