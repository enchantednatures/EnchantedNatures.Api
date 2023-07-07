use sqlx::PgPool;
use anyhow::Result;
use sqlx::types::chrono::{DateTime, Utc};

pub struct Photo {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct Category {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
}

pub struct PhotoCategory {
    pub id: i32,
    pub display_order: i32,
    pub photo_id: i32,
    pub category_id: i32,
}

pub struct Repository {
    pg_pool: PgPool,
}

impl Repository {
    pub fn new(pg_pool: PgPool) -> Self {
        Self { pg_pool }
    }

    pub async fn get_all(&self) -> Result<Vec<Photo>> {
        let photos = sqlx::query_as!(Photo, r#"
SELECT id as "id!",
       name as "name!",
       description as "description!",
       url as "url!",
       created_at,
       updated_at
FROM public.photos
        "#)
            .fetch_all(&self.pg_pool)
            .await?;
        Ok(photos)
    }
}