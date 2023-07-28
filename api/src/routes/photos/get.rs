use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{response, Json};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PhotoGetAllResponse;

#[utoipa::path(
    get,
    path = "/api/v0/photos/",
    responses(
        (status = StatusCode::OK, description = "Get all photos", body = [Photo]),
    )
)]
pub async fn get_photos(
    State(db_pool): State<PgPool>,
) -> response::Result<impl IntoResponse, (StatusCode, String)> {
    // let mut response: Vec<Photo> = vec![];
    let response = sqlx::query_file_as!(Photo, "sql/get_photos.sql")
        .fetch_all(&db_pool)
        .await
        .unwrap();
    Ok((StatusCode::OK, Json(response)))
}
