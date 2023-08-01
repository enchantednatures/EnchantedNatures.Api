use crate::database::PhotoRepo;
use crate::Database;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::Photo;
use crate::requests::PhotoCreateRequest;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PhotoCreatedResponse {
    pub photo: Photo,
}

#[utoipa::path(
    post,
    path = "/api/v0/photos/",
    request_body = PhotoCreateRequest,
    responses(
        (status = 201, description = "photo created successfully", body = Photo),
        (status = 409, description = "photo already exists", body = PhotoError),
    )
)]
pub async fn post_photo(
    Extension(repo): Extension<Database>,
    Json(payload): Json<PhotoCreateRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let photo = repo
        .add_photo(payload.name, payload.description, payload.url)
        .await
        .unwrap();

    Ok((StatusCode::CREATED, Json(photo)))
}
