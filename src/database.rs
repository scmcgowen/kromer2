pub mod name;
pub mod player;
pub mod transaction;
pub mod wallet;

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
