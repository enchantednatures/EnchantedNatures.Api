use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;
use sqlx::PgPool;

use crate::models::Photo;

#[mockall::automock]
#[async_trait]
pub trait PhotoRepo {
    async fn add_photo(&self, name: String, description: String, url: String) -> Result<Photo>;
    async fn get_photo(&self, id: i32) -> Result<Photo>;
    async fn get_photos_in_category(&self, id: i32) -> Result<Vec<Photo>>;
    async fn get_photos(&self) -> Result<Vec<Photo>>;

    async fn add_category(&self, name: String, description: String) -> Result<i32>;
}

pub struct PhotoRepository {
    db_pool: Arc<PgPool>,
}

impl PhotoRepository {
    fn new(pg_pool: PgPool) -> Self {
        PhotoRepository {
            db_pool: Arc::new(pg_pool),
        }
    }
}
#[async_trait]
impl PhotoRepo for PhotoRepository {
    async fn add_photo(&self, name: String, description: String, url: String) -> Result<Photo> {
        let response = sqlx::query_as!(
            Photo,
            r#"
        INSERT INTO public.photos (name, description, url)
        VALUES ($1, $2, $3) RETURNING id as "id!",
               name as "name!",
               description as "description!",
               url as "url!",
               created_at as "created_at!",
               updated_at as "updated_at!"
        "#,
            name,
            description,
            url
        )
        .fetch_one(&*self.db_pool)
        .await
        .unwrap();
        Ok(response)
    }
    async fn get_photo(&self, id: i32) -> Result<Photo> {
        todo!()
    }
    async fn get_photos_in_category(&self, id: i32) -> Result<Vec<Photo>> {
        todo!()
    }

    async fn get_photos(&self) -> Result<Vec<Photo>> {
        let response = sqlx::query_as!(
            Photo,
            r#"
            SELECT id as "id!",
               name as "name!",
               url as "url!",
               description as "description!",
               created_at as "created_at!",
               updated_at as "updated_at!"
            FROM public.photos "#
        )
        .fetch_all(&*self.db_pool)
        .await
        .unwrap();
        Ok(response)
    }

    async fn add_category(&self, name: String, description: String) -> Result<i32> {
        todo!()
    }
}
