use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;

use crate::models::Photo;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PhotoCreateRequest {
    pub name: String,
    pub description: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PhotoCreatedResponse {
    pub photo: Photo,
}

#[utoipa::path(
    put,
    path = "/api/v0/photos/",
    request_body = PhotoCreateRequest,
    responses(
        (status = 201, description = "photo created successfully", body = Photo),
        (status = 409, description = "photo already exists", body = PhotoError),
    )
)]
pub async fn put_photo(
    State(db_pool): State<PgPool>,
    Json(payload): Json<PhotoCreateRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let photo = sqlx::query_as!(
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
        payload.name,
        payload.description,
        payload.url
    )
    .fetch_one(&db_pool)
    .await
    .unwrap();

    Ok((StatusCode::CREATED, Json(photo)))
}
