use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{response, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub enum HealthStatusEnum {
    Ok,
    Error,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct HealthStatus {
    status: HealthStatusEnum,
}

impl HealthStatus {
    pub(crate) fn new() -> Self {
        HealthStatus {
            status: HealthStatusEnum::Ok,
        }
    }
}

#[utoipa::path(
    get,
    path = "/health_check",
    responses(
        (status = StatusCode::OK, description = "Check health", body = HealthStatus)
    )
)]
pub async fn health_check() -> response::Result<impl IntoResponse, (StatusCode, String)> {
    Ok(Json(HealthStatus::new()))
}
