use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;
use sqlx::{query_file, query_file_as, PgPool};

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
    async fn delete_photo(&self, id: i32) -> Result<()>;

    async fn add_category(&self, name: String, description: String) -> Result<Category>;
    async fn get_category(&self, id: i32) -> Result<Category>;
    async fn get_categories(&self) -> Result<Vec<Category>>;
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
        let response = sqlx::query_file_as!(Photo, "sql/photos/insert.sql", name, description, url)
            .fetch_one(&*self.db_pool)
            .await?;
        Ok(response)
    }

    async fn get_photo(&self, id: i32) -> Result<Photo> {
        let response = query_file_as!(Photo, "sql/photos/get.sql", id)
            .fetch_one(&*self.db_pool)
            .await?;
        Ok(response)
    }

    async fn get_photos_in_category(&self, id: i32) -> Result<Vec<Photo>> {
        let response = query_file_as!(Photo, "sql/photo_categories/get.sql", id)
            .fetch_all(&*self.db_pool)
            .await?;
        Ok(response)
    }

    async fn get_photos(&self) -> Result<Vec<Photo>> {
        let response = query_file_as!(Photo, "sql/photos/get_all.sql")
            .fetch_all(&*self.db_pool)
            .await?;
        Ok(response)
    }

    async fn delete_photo(&self, id: i32) -> Result<()> {
        query_file!("sql/photos/delete.sql", id)
            .execute(&*self.db_pool)
            .await?;
        Ok(())
    }

    async fn add_category(&self, name: String, description: String) -> Result<Category> {
        let response = query_file_as!(Category, "sql/categories/insert.sql", name, description)
            .fetch_one(&*self.db_pool)
            .await?;
        Ok(response)
    }

    async fn get_category(&self, id: i32) -> Result<Category> {
        let response = query_file_as!(Category, "sql/categories/get.sql", id)
            .fetch_one(&*self.db_pool)
            .await?;
        Ok(response)
    }

    async fn get_categories(&self) -> Result<Vec<Category>> {
        let response = query_file_as!(Category, "sql/categories/get_all.sql")
            .fetch_all(&*self.db_pool)
            .await?;
        Ok(response)
    }
}
