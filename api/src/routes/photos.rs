use crate::models::Category;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{response, Json};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::Utc;
use utoipa::ToSchema;

#[utoipa::path(get, path = "/categories/", responses(
(status = StatusCode::OK, description = "Check health", body = [Category]),))]
pub async fn get_categories() -> response::Result<impl IntoResponse, (StatusCode, String)> {
    let mut response: Vec<Category> = vec![];
    response.push(Category::new(
        -1,
        "test".to_string(),
        "test".to_string(),
        Utc::now(),
        Utc::now(),
    ));
    Ok(Json(response))
}

#[utoipa::path(get, path = "/categories/{id}/", responses((status = StatusCode::OK, description = "Check health", body = HealthStatus),))]
pub async fn get_photos_in_category() {
    todo!()
}
