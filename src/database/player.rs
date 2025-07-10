use async_trait::async_trait;
use sqlx::{MySql, Pool};
use uuid::Uuid;

use crate::database::ModelExt;

#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct Model {
    pub id: Uuid,
    pub name: String,
}

#[async_trait]
impl ModelExt for Model {
    async fn fetch_by_id(pool: &Pool<MySql>, id: i64) -> sqlx::Result<Option<Self>>
    where
        Self: Sized,
    {
        let q = "SELECT * FROM transaction WHERE id = ?";

        sqlx::query_as(q).bind(id).fetch_optional(pool).await
    }

    async fn fetch_all(pool: &Pool<MySql>, limit: u64, offset: u64) -> sqlx::Result<Vec<Self>>
    where
        Self: Sized,
    {
        let limit = limit.clamp(0, 1000);
        let q = "SELECT * from transaction LIMIT ? OFFSET ?";

        sqlx::query_as(q)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await
    }

    async fn total_count(pool: &Pool<MySql>) -> sqlx::Result<usize> {
        let q = "SELECT COUNT() FROM transaction";
        let result: i64 = sqlx::query_scalar(q).fetch_one(pool).await?;

        Ok(result as usize)
    }
}
