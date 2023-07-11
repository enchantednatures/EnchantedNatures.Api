use crate::models::{Category, Photo};
use axum::extract::State;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Json, response};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CategoryGetByIdRequest {
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CategoryGetByIdResponse {
    pub category: Category,
    pub photos: Vec<Photo>,
}

#[utoipa::path(get, path = "/categories/{id}",
responses(
(status = StatusCode::OK, description = " health", body = CategoryGetByIdResponse),))]
pub async fn get_category_by_id(
    State(db_pool): State<PgPool>,
    Path(id): Path<i32>,
) -> response::Result<impl IntoResponse, (StatusCode, String)> {
    let category = sqlx::query_as!(
        Category,
        r#"
            SELECT id as "id!",
               name as "name!",
               description as "description!",
               created_at as "created_at!",
               updated_at as "updated_at!"
            FROM public.categories
            WHERE id = $1
            "#,
        id
    )
        .fetch_one(&db_pool)
        .await
        .unwrap();

    let photos = sqlx::query_as!(
        Photo,
        r#"
            SELECT id as "id!",
               name as "name!",
               description as "description!",
               url as "url!",
               created_at as "created_at!",
               updated_at as "updated_at!"
            FROM public.photos
            JOIN public.photo_categories ON photos.id = photo_categories.photo_id
            WHERE photo_categories.category_id = $1
            ORDER BY photo_categories.display_order DESC
            "#,
        id
    )
        .fetch_all(&db_pool)
        .await
        .unwrap();

    Ok((StatusCode::OK, Json(CategoryGetByIdResponse {
        category,
        photos,
    })))
}
