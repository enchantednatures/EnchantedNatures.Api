use crate::models;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{response, Json};
use models::Photo;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PhotoGetAllResponse;

#[utoipa::path(get, path = "/api/v0/photos/",
responses(
(status = StatusCode::OK, description = "Get all photos", body = [Photo]),))]
pub async fn get_photos(
    State(db_pool): State<PgPool>,
) -> response::Result<impl IntoResponse, (StatusCode, String)> {
    // let mut response: Vec<Photo> = vec![];
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
    .fetch_all(&db_pool)
    .await
    .unwrap();
    Ok((StatusCode::OK, Json(response)))
}

#[utoipa::path(get, path = "/categories/{id}/", responses((status = StatusCode::OK, description = "Check health", body = HealthStatus),))]
pub async fn get_photos_in_category(
    State(db_pool): State<PgPool>,
    Path(id): Path<i32>,
) -> response::Result<impl IntoResponse, (StatusCode, String)> {
    Ok((StatusCode::OK, Json(json!({"status":"not implemented"}))))
}
