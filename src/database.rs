pub mod name;
pub mod player;
pub mod transaction;
pub mod wallet;

use sqlx::{Encode, Executor, Postgres, prelude::Type};

#[async_trait::async_trait]
pub trait ModelExt<'q> {
    /// Fetches a record from a table and returns it
    async fn fetch_by_id<T, E>(pool: E, id: T) -> sqlx::Result<Option<Self>>
    where
        Self: Sized,
        T: 'q + Encode<'q, Postgres> + Type<Postgres> + Send,
        E: 'q + Executor<'q, Database = Postgres>;

    /// Fetches all records from a table and returns them
    async fn fetch_all<E>(pool: E, limit: i64, offset: i64) -> sqlx::Result<Vec<Self>>
    where
        Self: Sized,
        E: 'q + Executor<'q, Database = Postgres>;

    /// Fetches the total number of records in the table
    async fn total_count<E>(pool: E) -> sqlx::Result<usize>
    where
        E: 'q + Executor<'q, Database = Postgres>;
}
