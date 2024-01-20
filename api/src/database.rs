use std::sync::Arc;

use anyhow::Result;
use chrono::NaiveDate;
use sqlx::PgPool;

use crate::models::{Category, CategoryPhotos, Photo};

#[derive(sqlx::Type)]
#[sqlx(transparent)]
struct Id(i32);

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

    pub async fn add_photo(
        &self,
        title: String,
        filename: String,
        location_taken: String,
        date_taken: NaiveDate,
    ) -> Result<Photo> {
        let response = sqlx::query_as!(
            Photo,
            r#"
                INSERT INTO photos (title, filename, location_taken, date_taken)
                VALUES ($1, $2, $3, $4) RETURNING id as "id!",
                    title as "title!",
                    filename as "filename!",
                    location_taken as "location_taken!",
                    date_taken as "date_taken!",
                    created_at as "created_at!",
                    updated_at as "updated_at!"
                    
            "#,
            title,
            filename,
            location_taken,
            date_taken
        )
        .fetch_one(&*self.db_pool)
        .await?;
        Ok(response)
    }

    pub async fn get_photo(&self, id: i32) -> Result<Photo> {
        let response = sqlx::query_as!(
            Photo,
            r#"
                SELECT id as "id!",
                    title as "title!",
                    filename as "filename!",
                    location_taken as "location_taken!",
                    date_taken as "date_taken!",
                    created_at as "created_at!",
                    updated_at as "updated_at!"
                FROM photos
                WHERE id = $1;
            "#,
            id
        )
        .fetch_one(&*self.db_pool)
        .await?;
        Ok(response)
    }

    pub async fn update_photo(
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

    pub async fn get_photos_in_category(&self, id: i32) -> Result<Vec<Photo>> {
        let response = sqlx::query_as!(
            Photo,
            r#"
                SELECT p.id          as "id!",
                    p.title as "title!",
                    p.filename as "filename!",
                    p.location_taken as "location_taken!",
                    p.date_taken as "date_taken!",
                    p.created_at as "created_at!",
                    p.updated_at as "updated_at!"
                FROM categories
                        JOIN photo_categories pc on categories.id = pc.category_id
                        JOIN photos p on p.id = pc.photo_id
                WHERE category_id = $1
            "#,
            id
        )
        .fetch_all(&*self.db_pool)
        .await?;
        Ok(response)
    }

    pub async fn get_photos(&self) -> Result<Vec<Photo>> {
        let response = sqlx::query_as!(
            Photo,
            r#"
                SELECT id as "id!",
                    title as "title!",
                    filename as "filename!",
                    location_taken as "location_taken!",
                    date_taken as "date_taken!",
                    created_at as "created_at!",
                    updated_at as "updated_at!"
                FROM photos
            "#
        )
        .fetch_all(&*self.db_pool)
        .await?;
        Ok(response)
    }

    pub async fn delete_photo(&self, id: i32) -> Result<()> {
        sqlx::query!(
            r#"
                DELETE
                FROM photos
                WHERE id = $1
            "#,
            id
        )
        .execute(&*self.db_pool)
        .await?;
        Ok(())
    }

    pub async fn add_photo_to_category(
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

    pub async fn add_category(&self, name: String) -> Result<Category> {
        let response = sqlx::query_as!(
            Category,
            r#"
                INSERT INTO categories (name)
                VALUES ($1)
                RETURNING id as "id!",
                        name as "name!",
                        created_at as "created_at!",
                        updated_at as "updated_at!";
            "#,
            name
        )
        .fetch_one(&*self.db_pool)
        .await?;
        Ok(response)
    }

    pub async fn get_category(&self, id: i32) -> Result<CategoryPhotos> {
        let response = sqlx::query_as!(
            Category,
            r#"
                SELECT id as "id!",
                    name as "name!",
                    created_at as "created_at!",
                    updated_at as "updated_at!"
                FROM categories
                WHERE id = $1;
            "#,
            id
        )
        .fetch_one(&*self.db_pool)
        .await?;
        let photos_in_category = sqlx::query_as!(
            Photo,
            r#"
                SELECT p.id          as "id!",
                    p.title as "title!",
                    p.filename as "filename!",
                    p.location_taken as "location_taken!",
                    p.date_taken as "date_taken!",
                    p.created_at as "created_at!",
                    p.updated_at as "updated_at!"
                FROM categories
                        JOIN photo_categories pc on categories.id = pc.category_id
                        JOIN photos p on p.id = pc.photo_id
                WHERE category_id = $1
                "#,
            id
        )
        .fetch_all(&*self.db_pool)
        .await?;
        Ok((response, photos_in_category))
    }

    pub async fn get_categories(&self) -> Result<Vec<Category>> {
        let response = sqlx::query_as!(
            Category,
            r#"
                SELECT id as "id!",
                    name as "name!",
                    created_at as "created_at!",
                    updated_at as "updated_at!"
                FROM categories;
            "#
        )
        .fetch_all(&*self.db_pool)
        .await?;
        Ok(response)
    }
}
