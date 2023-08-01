use crate::database::PhotoRepo;
use crate::models::{Photo, PhotoViewModel};
use crate::Database;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{response, Extension, Json};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::{IntoParams, IntoResponses, ToSchema};

#[derive(Debug, Serialize, Deserialize, IntoResponses)]
pub enum GetPhotosResponses {
    #[response(status = StatusCode::OK, description = "Get all photos")]
    Success(Vec<Photo>),
}

#[utoipa::path(get, path = "/api/v0/photos/", responses(GetPhotosResponses))]
#[tracing::instrument(name = "Get photos", skip(db_pool))]
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

#[derive(Debug, Serialize, Deserialize, IntoResponses)]
pub enum GetPhotoResponses {
    #[response(status = StatusCode::OK, description = "Get photo by id")]
    Success(PhotoViewModel),

    #[response(status = StatusCode::NOT_FOUND, description = "Unable to find Photo")]
    NotFound,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
#[into_params(parameter_in = Path)]
pub struct GetPhotoParams {
    // PhotoId
    id: i32,
}

#[utoipa::path(
    get,
    path = "/api/v0/photos/{id}",
    params(
        ("id"= i32, Path, description = "Photo Id")
    ),
    responses(GetPhotoResponses)
)]
#[tracing::instrument(name = "Get photos", skip(repo))]
pub async fn get_photo(
    Extension(repo): Extension<Database>,
    Path(id): Path<i32>,
) -> response::Result<impl IntoResponse, (StatusCode, String)> {
    match repo.get_photo(id).await {
        Ok(photo) => {
            let view_model: PhotoViewModel = photo.into();
            Ok((StatusCode::OK, Json(view_model)))
        }
        Err(_) => Err((
            StatusCode::NOT_FOUND,
            format!("Photo with id: {} not found", id),
        )),
    }
}
