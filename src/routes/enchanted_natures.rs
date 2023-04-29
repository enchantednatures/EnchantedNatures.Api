use crate::view_models::{CategoryPhotoViewModel, CategoryViewModel, PhotoViewModel};
use actix_web::{
    web::{self, Data, Path},
    HttpResponse, Responder,
};
use paperclip::actix::web::Json;
use sqlx::{Error, PgPool};
use uuid::Uuid;

use actix_web::{web, HttpRequest, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;
// Create a new photo

#[post("/photos")]
pub async fn create_photo(
    pool: &PgPool,
    id: Uuid,
    description: String,
    date_taken: chrono::NaiveDateTime,
    cdn_path: String,
) -> Result<Photo, sqlx::Error> {
    let query = include_str!("../sql/create_photo.sql");
    sqlx::query_as::<_, Photo>(query)
        .bind(id)
        .bind(description)
        .bind(date_taken)
        .bind(cdn_path)
        .fetch_one(pool)
        .await
}

// Get all photos
#[get("/photos")]
async fn get_photos(pool: Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as!(
        PhotoViewModel,
        r#"
            SELECT id, description, date_taken, cdn_path
            FROM photos
        "#
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(photos) => HttpResponse::Ok().json(photos),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Get a specific photo by id
#[get("/photos/{id}")]
async fn get_photo_by_id(pool: Data<PgPool>, id: Path<Uuid>) -> impl Responder {
    let query = include_str!("../sql/get_photo_by_id.sql");
    let result = sqlx::query_as!(
        PhotoViewModel,
        query
        id.into_inner()
    )
    .fetch_optional(pool.get_ref())
    .await;

    match result {
        Ok(Some(photo)) => HttpResponse::Ok().json(photo),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Update a specific photo by id
#[put("/photos/{id}")]
async fn update_photo(
    pool: Data<PgPool>,
    id: Path<Uuid>,
    photo: Json<PhotoViewModel>,
) -> impl Responder {
    let result = sqlx::query!(
        r#"
            UPDATE photos
            SET description = $2, date_taken = $3, cdn_path = $4
            WHERE id = $1
        "#,
        id.into_inner(),
        photo.description,
        photo.date_taken,
        photo.cdn_path
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Delete a specific photo by id
#[delete("/photos/{id}")]
async fn delete_photo(pool: Data<PgPool>, id: Path<Uuid>) -> impl Responder {
    let result = sqlx::query!(
        r#"
            DELETE FROM photos
            WHERE id = $1
        "#,
        id.into_inner()
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/categories/{category_id}/photos/{photo_id}")]
async fn add_photo_to_category(
    pool: Data<PgPool>,
    path: Path<(Uuid, Uuid)>,
    order: Json<i32>,
) -> impl Responder {
    let (category_id, photo_id) = path.into_inner();
    let order = order.into_inner();

    let mut tx = pool.begin().await.unwrap();

    let update_result = sqlx::query!(
        r#"
        UPDATE category_photos
        SET ordering = ordering + 1
        WHERE category_id = $1 AND ordering >= $2
        "#,
        category_id,
        order
    )
    .execute(&mut tx)
    .await;

    let insert_result = sqlx::query!(
        r#"
        INSERT INTO category_photos (category_id, photo_id, ordering)
        VALUES ($1, $2, $3)
        "#,
        category_id,
        photo_id,
        order
    )
    .execute(&mut tx)
    .await;

    match (update_result, insert_result) {
        (Ok(_), Ok(_)) => {
            tx.commit().await.unwrap();
            HttpResponse::Created().finish()
        }
        _ => {
            tx.rollback().await.unwrap();
            HttpResponse::InternalServerError().finish()
        }
    }
}
// Get photos in a category
#[get("/categories/{category_id}/photos")]
async fn get_photos_by_category(pool: Data<PgPool>, category_id: Path<Uuid>) -> impl Responder {
    let result = sqlx::query!(
        r#"
            SELECT p.id, p.description, p.date_taken, p.cdn_path
            FROM photos p
            JOIN category_photos cp ON cp.photo_id = p.id
            WHERE cp.category_id = $1
            ORDER BY cp.order_in_category
        "#,
        category_id.into_inner()
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(photos) => HttpResponse::Ok().json(photos),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Remove a photo from a category
#[delete("/categories/{category_id}/photos/{photo_id}")]
async fn remove_photo_from_category(
    pool: Data<PgPool>,
    path: Path<(Uuid, Uuid)>,
) -> impl Responder {
    let (category_id, photo_id) = path.into_inner();
    let result = sqlx::query!(
        r#"
            DELETE FROM category_photos
            WHERE category_id = $1 AND photo_id = $2
        "#,
        category_id,
        photo_id
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Configure the routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_categories);
    cfg.service(put_category);
    cfg.service(create_photo);
    cfg.service(get_photos);
    cfg.service(get_photo_by_id);
    cfg.service(update_photo);
    cfg.service(delete_photo);
    cfg.service(add_photo_to_category);
    cfg.service(get_photos_by_category);
    cfg.service(remove_photo_from_category);
}