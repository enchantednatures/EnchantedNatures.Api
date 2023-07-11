use axum::extract::State;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Json, response};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct DeleteCategoryRequest;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct DeleteCategoryResponse {
    status: String,
}
#[utoipa::path(delete, path = "/categories/{id}",
responses(
(status = StatusCode::OK, description = " health", body = DeleteCategoryResponse),))]
async fn delete(
    State(db_pool): State<PgPool>,
    Path(id): Path<i32>,
) -> response::Result<impl IntoResponse, (StatusCode, String)> {
    Ok((StatusCode::OK, Json(json!({"status": "not implemented"}))))
}