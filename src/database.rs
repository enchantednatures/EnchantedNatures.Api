use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;
use sqlx::PgPool;

use crate::models::{Category, Photo};

#[derive(sqlx::Type)]
#[sqlx(transparent)]
struct Id(i32);

#[mockall::automock]
#[async_trait]
pub trait PhotoRepo {
    async fn add_photo(&self, name: String, description: String, url: String) -> Result<Photo>;
    async fn get_photo(&self, id: i32) -> Result<Photo>;
    async fn get_photos_in_category(&self, id: i32) -> Result<Vec<Photo>>;
    async fn get_photos(&self) -> Result<Vec<Photo>>;

    async fn add_category(&self, name: String, description: String) -> Result<Category>;
}

pub struct PhotoRepository {
    db_pool: Arc<PgPool>,
}

impl PhotoRepository {
    pub fn new(pg_pool: PgPool) -> Self {
        PhotoRepository {
            db_pool: Arc::new(pg_pool),
        }
    }
}

#[async_trait]
impl PhotoRepo for PhotoRepository {
    async fn add_photo(&self, name: String, description: String, url: String) -> Result<Photo> {
        let response = sqlx::query_file_as!(Photo, "sql/photo_insert.sql", name, description, url)
            .fetch_one(&*self.db_pool)
            .await?;
        Ok(response)
    }

    async fn get_photo(&self, id: i32) -> Result<Photo> {
        let response = sqlx::query_file_as!(Photo, "sql/photos/get.sql", id)
            .fetch_one(&*self.db_pool)
            .await?;
        return Ok(response);
    }

    async fn get_photos_in_category(&self, _id: i32) -> Result<Vec<Photo>> {
        todo!()
    }

    async fn get_photos(&self) -> Result<Vec<Photo>> {
        let response = sqlx::query_file_as!(Photo, "sql/photos/get_all.sql")
            .fetch_all(&*self.db_pool)
            .await
            .unwrap();
        Ok(response)
    }

    async fn add_category(&self, name: String, description: String) -> Result<Category> {
        let response =
            sqlx::query_file_as!(Category, "sql/categories/insert.sql", name, description)
                .fetch_one(&*self.db_pool)
                .await?;
        Ok(response)
    }
}
