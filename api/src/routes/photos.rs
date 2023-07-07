use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{response, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


pub async fn get_categories() {
    todo!()
}


#[utoipa::path(get, path = "/categories/{id}/", responses((status = StatusCode::OK, description = "Check health", body = HealthStatus),))]
pub async fn get_photos_in_category() {
    todo!()
}
