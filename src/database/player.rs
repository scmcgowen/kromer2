use async_trait::async_trait;
use sqlx::{Encode, Pool, Postgres, Type};
use uuid::Uuid;

use crate::database::ModelExt;

#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct Model {
    pub id: Uuid,
    pub name: String,
}

#[async_trait]
impl<'q> ModelExt<'q> for Model {
    async fn fetch_by_id<T: 'q + Encode<'q, Postgres> + Type<Postgres> + Send>(
        pool: &Pool<Postgres>,
        id: T,
    ) -> sqlx::Result<Option<Model>> {
        let q = "SELECT * FROM players WHERE id = $1";

        sqlx::query_as(q).bind(id).fetch_optional(pool).await
    }

    async fn fetch_all(pool: &Pool<Postgres>, limit: i64, offset: i64) -> sqlx::Result<Vec<Self>>
    where
        Self: Sized,
    {
        let limit = limit.clamp(0, 1000);
        let q = "SELECT * from players LIMIT $1 OFFSET $2";

        sqlx::query_as(q)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await
    }

    async fn total_count(pool: &Pool<Postgres>) -> sqlx::Result<usize> {
        let q = "SELECT COUNT() FROM players";
        let result: i64 = sqlx::query_scalar(q).fetch_one(pool).await?;

        Ok(result as usize)
    }
}
