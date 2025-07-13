pub mod name;
pub mod player;
pub mod transaction;
pub mod wallet;

use sqlx::{Encode, Pool, Postgres, prelude::Type};

#[async_trait::async_trait]
pub trait ModelExt<'q> {
    /// Fetches a record from a table and returns it
    async fn fetch_by_id<T: 'q + Encode<'q, Postgres> + Type<Postgres> + Send>(
        pool: &Pool<Postgres>,
        id: T,
    ) -> sqlx::Result<Option<Self>>
    where
        Self: Sized;

    /// Fetches all records from a table and returns them
    async fn fetch_all(pool: &Pool<Postgres>, limit: i64, offset: i64) -> sqlx::Result<Vec<Self>>
    where
        Self: Sized;

    /// Fetches the total number of records in the table
    async fn total_count(pool: &Pool<Postgres>) -> sqlx::Result<usize>;
}
