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

    async fn add_photo_to_category(
        &self,
        photo_id: i32,
        category_id: i32,
        display_order: Option<i32>,
    ) -> Result<()>;

    async fn add_category(&self, name: String, description: String) -> Result<Category>;
    async fn get_category(&self, id: i32) -> Result<Category>;
    async fn get_categories(&self) -> Result<Vec<Category>>;
}

pub struct PhotoRepository {
    pub db_pool: Arc<PgPool>,
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

    async fn add_photo_to_category(
        &self,
        photo_id: i32,
        category_id: i32,
        display_order: Option<i32>,
    ) -> Result<()> {
        let mut transaction = self.db_pool.begin().await?;

        let row = sqlx::query!(
        r#"SELECT MAX(display_order) as max_display_order FROM photo_categories WHERE category_id = $1"#,
        &category_id
    )
        .fetch_one(&mut *transaction)
        .await
        .unwrap();

        let max_value = row.max_display_order.unwrap_or(0); // Use 0 if there are no rows

        if let Some(display) = display_order {
            if display <= max_value {
                sqlx::query!(
                    r#"
                UPDATE photo_categories
                SET display_order = display_order + 1
                WHERE category_id = $1
                AND display_order > $2
           "#,
                    &category_id,
                    &display
                )
                .execute(&mut *transaction)
                .await
                .unwrap();
            }
        }

        // Now insert a new row with `max_value + 1` as the value for `your_field`
        // Replace "your_values" with the values for the other fields in your table
        sqlx::query!(
        "INSERT INTO photo_categories (category_id, photo_id, display_order) VALUES ($1, $2, $3)",
        &category_id,
        &photo_id,
        &max_value + 1
    )
        .execute(&mut *transaction)
        .await
        .unwrap();

        transaction.commit().await.unwrap();
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
