
use crate::view_models::{CategoryViewModel, PhotoViewModel, CategoryPhotoViewModel};
use actix_web::{web, HttpResponse, Responder};
use sqlx::{PgPool, Error};
use uuid::Uuid;

// ... other routes ...

// Create a new photo
#[post("/photos")]
async fn create_photo(pool: web::Data<PgPool>, photo: web::Json<PhotoViewModel>) -> impl Responder {
    let id = Uuid::new_v4();
    let result = sqlx::query!(
        r#"
            INSERT INTO photos (id, description, date_taken, cdn_path)
            VALUES ($1, $2, $3, $4)
        "#,
        id,
        photo.description,
        photo.date_taken,
        photo.cdn_path
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Created().json(PhotoViewModel {
            id: Some(id),
            description: photo.description.clone(),
            date_taken: photo.date_taken,
            cdn_path: photo.cdn_path.clone(),
        }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Get all photos
#[get("/photos")]
async fn get_photos(pool: web::Data<PgPool>) -> impl Responder {
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
async fn get_photo_by_id(pool: web::Data<PgPool>, id: web::Path<Uuid>) -> impl Responder {
    let result = sqlx::query_as!(
        PhotoViewModel,
        r#"
            SELECT id, description, date_taken, cdn_path
            FROM photos
            WHERE id = $1
        "#,
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
async fn update_photo(pool: web::Data<PgPool>, id: web::Path<Uuid>, photo: web::Json<PhotoViewModel>) -> impl Responder {
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
async fn delete_photo(pool: web::Data<PgPool>, id: web::Path<Uuid>) -> impl Responder {
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

// Add a photo to a category
#[post("/categories/{category_id}/photos/{photo_id}")]
async fn add_photo_to_category(pool: web::Data<PgPool>, path: web::Path<(Uuid, Uuid)>, order: web::Json<i32>) -> impl Responder {
    let (category_id, photo_id) = path.into_inner();
    let result = sqlx::query!(
        r#"
            INSERT INTO category_photos (category_id, photo_id, order_in_category)
            VALUES ($1, $2, $3)
        "#,
        category_id,
        photo_id,
        order.into_inner()
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Get photos in a category
#[get("/categories/{category_id}/photos")]
async fn get_photos_by_category(pool: web::Data<PgPool>, category_id: web::Path<Uuid>) -> impl Responder {
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
async fn remove_photo_from_category(pool: web::Data<PgPool>, path: web::Path<(Uuid, Uuid)>) -> impl Responder {
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
