use crate::models::Photo;
use anyhow::Result;
use sqlx::PgPool;

pub struct Repository {
    pg_pool: PgPool,
}

impl Repository {
    pub fn new(pg_pool: PgPool) -> Self {
        Self { pg_pool }
    }

    pub async fn get_all(&self) -> Result<Vec<Photo>> {
        let photos = sqlx::query_as!(
            Photo,
            r#"
SELECT id as "id!",
       name as "name!",
       description as "description!",
       url as "url!",
       created_at,
       updated_at
FROM public.photos
        "#
        )
        .fetch_all(&self.pg_pool)
        .await?;
        Ok(photos)
    }
}
