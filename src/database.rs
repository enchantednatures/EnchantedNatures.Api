use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;
use chrono::NaiveDate;
use sqlx::{query_file, query_file_as, PgPool};

use crate::models::{Category, CategoryPhotos, Photo};

#[derive(sqlx::Type)]
#[sqlx(transparent)]
struct Id(i32);

#[async_trait]
pub trait PhotoRepo {
    async fn add_photo(
        &self,
        title: String,
        filename: String,
        location_taken: String,
        date_taken: NaiveDate,
    ) -> Result<Photo>;
    async fn get_photo(&self, id: i32) -> Result<Photo>;
    async fn update_photo(
        &self,
        id: i32,
        title: Option<String>,
        filename: Option<String>,
        location_taken: Option<String>,
        date_taken: Option<NaiveDate>,
    ) -> Result<Photo>;
    async fn get_photos_in_category(&self, id: i32) -> Result<Vec<Photo>>;
    async fn get_photos(&self) -> Result<Vec<Photo>>;
    async fn delete_photo(&self, id: i32) -> Result<()>;

    async fn add_photo_to_category(
        &self,
        photo_id: i32,
        category_id: i32,
        display_order: Option<i32>,
    ) -> Result<()>;

    async fn add_category(&self, name: String) -> Result<Category>;
    async fn get_category(&self, id: i32) -> Result<CategoryPhotos>;
    async fn get_categories(&self) -> Result<Vec<Category>>;
}

#[derive(Debug, Clone)]
pub struct PhotoRepository {
    pub db_pool: Arc<PgPool>,
}

impl PhotoRepository {
    pub fn new(pg_pool: PgPool) -> Self {
        PhotoRepository {
            db_pool: Arc::new(pg_pool),
        }
    }
    pub async fn migrate(&self) -> Result<()> {
        sqlx::migrate!().run(&*self.db_pool).await?;
        Ok(())
    }
}

#[async_trait]
impl PhotoRepo for PhotoRepository {
    async fn add_photo(
        &self,
        title: String,
        filename: String,
        location_taken: String,
        date_taken: NaiveDate,
    ) -> Result<Photo> {
        let response = sqlx::query_file_as!(
            Photo,
            "sql/photos/insert.sql",
            title,
            filename,
            location_taken,
            date_taken
        )
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

    async fn update_photo(
        &self,
        id: i32,
        title: Option<String>,
        filename: Option<String>,
        location_taken: Option<String>,
        date_taken: Option<NaiveDate>,
    ) -> Result<Photo> {
        match self.get_photo(id).await {
            Ok(photo) => {
                let response = sqlx::query_as!(
                    Photo,
                    r#"
                    UPDATE photos
                    SET title = $2,
                        filename = $3,
                        location_taken = $4,
                        date_taken = $5
                    WHERE 
                        id = $1
                    RETURNING 
                        id as "id!",
                        title as "title!",
                        filename as "filename!",
                        location_taken as "location_taken!",
                        date_taken as "date_taken!",
                        created_at as "created_at!",
                        updated_at as "updated_at!"
                    "#,
                    id,
                    title.unwrap_or(photo.title),
                    filename.unwrap_or(photo.filename),
                    location_taken.unwrap_or(photo.location_taken),
                    date_taken.unwrap_or(photo.date_taken)
                )
                .fetch_one(&*self.db_pool)
                .await?;
                Ok(response)
            }
            Err(e) => Err(e),
        }
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

        if let Some(display_order) = display_order {
            let category_has = sqlx::query_scalar!(
                r#"SELECT display_order as "display_order!"
            FROM photo_categories
            WHERE 
                category_id = $1
                AND
                display_order = $2
                "#,
                category_id,
                display_order
            )
            .fetch_optional(&mut *transaction)
            .await
            .unwrap();

            match category_has {
                Some(_) => todo!(),
                None => {
                    sqlx::query!(
                        r#"INSERT INTO photo_categories 
                    (category_id, photo_id, display_order) 
                    VALUES ($1, $2, $3)"#,
                        &category_id,
                        &photo_id,
                        &display_order
                    )
                    .execute(&mut *transaction)
                    .await
                    .unwrap();
                }
            };
        } else {
            let row = sqlx::query_scalar!(
                r#"SELECT 
                    MAX(display_order) as "max_display_order!"
               FROM photo_categories 
               WHERE 
                    category_id = $1
            "#,
                &category_id
            )
            .fetch_optional(&mut *transaction)
            .await
            .unwrap();
            let next_display_value = row.unwrap_or(0) + 1;

            sqlx::query!(
                "INSERT INTO photo_categories 
                (category_id, photo_id, display_order) 
                VALUES ($1, $2, $3)
",
                &category_id,
                &photo_id,
                &next_display_value
            )
            .execute(&mut *transaction)
            .await
            .unwrap();
        }

        // if let Some(display) = display_order {
        //     if display <= max_value {
        //         sqlx::query!(
        //             r#"
        //         UPDATE photo_categories
        //         SET display_order = display_order + 1
        //         WHERE category_id = $1
        //         AND display_order > $2
        //    "#,
        //             &category_id,
        //             &display
        //         )
        //         .execute(&mut *transaction)
        //         .await
        //         .unwrap();
        //     }
        // }

        transaction.commit().await.unwrap();
        Ok(())
    }

    async fn add_category(&self, name: String) -> Result<Category> {
        let response = query_file_as!(Category, "sql/categories/insert.sql", name)
            .fetch_one(&*self.db_pool)
            .await?;
        Ok(response)
    }

    async fn get_category(&self, id: i32) -> Result<CategoryPhotos> {
        let response = query_file_as!(Category, "sql/categories/get.sql", id)
            .fetch_one(&*self.db_pool)
            .await?;
        let photos_in_category = query_file_as!(Photo, "sql/photo_categories/get.sql", id)
            .fetch_all(&*self.db_pool)
            .await?;
        Ok((response, photos_in_category))
    }

    async fn get_categories(&self) -> Result<Vec<Category>> {
        let response = query_file_as!(Category, "sql/categories/get_all.sql")
            .fetch_all(&*self.db_pool)
            .await?;
        Ok(response)
    }
}
