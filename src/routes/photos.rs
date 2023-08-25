use crate::app::App;
use crate::database::PhotoRepo;
use crate::error_handling::AppError;
use crate::models::{Photo, PhotoViewModel};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{response, Json};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::info;

#[derive(Debug, Serialize, Deserialize)]
pub struct PhotoGetAllResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct PhotoCreateRequest {
    pub title: String,
    pub location_taken: String,
    pub date_taken: NaiveDate,
    pub filename: String,
}

#[tracing::instrument(name = "Delete photo", skip(app))]
pub async fn delete_photo(
    State(app): State<App>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    app.repo.delete_photo(id).await?;
    Ok((StatusCode::NO_CONTENT, Json(json!({ "deleted": &id }))))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhotoCreatedResponse {
    pub photo: Photo,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CreatePhotoResponses {
    Created(Photo),

    AlreadyExist,

    BadRequest,
}

#[tracing::instrument(name = "add photo", skip(app))]
pub async fn post_photo(
    State(app): State<App>,
    Json(payload): Json<PhotoCreateRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    info!("creating photo");
    println!("{:?}", payload);
    info!("{}", payload.title);
    let photo = app
        .repo
        .add_photo(
            payload.title,
            payload.filename,
            payload.location_taken,
            payload.date_taken,
        )
        .await
        .unwrap();

    info!("photo created");
    Ok((StatusCode::CREATED, Json(photo)))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhotoUpdateRequest {
    pub title: Option<String>,
    pub location_taken: Option<String>,
    pub date_taken: Option<NaiveDate>,
    pub filename: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UpdatePhotoResponses {
    Updated(Photo),

    DoesNotExist,

    BadRequest,
}

#[tracing::instrument(name = "update photo", skip(app))]
pub async fn put_photo(
    State(app): State<App>,
    Path(id): Path<i32>,
    Json(payload): Json<PhotoUpdateRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    info!("creating photo");
    println!("{:?}", payload);
    let photo = app
        .repo
        .update_photo(
            id,
            payload.title,
            payload.filename,
            payload.location_taken,
            payload.date_taken,
        )
        .await
        .unwrap();
    info!("photo Updated");
    Ok((StatusCode::OK, Json(photo)))
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GetPhotosResponses {
    Success(Vec<Photo>),
}

#[tracing::instrument(name = "Get photos", skip(app))]
pub async fn get_photos(
    State(app): State<App>,
) -> response::Result<impl IntoResponse, (StatusCode, String)> {
    info!("getting all photos");
    match app.repo.get_photos().await {
        Ok(response) => {
            info!("retrieved {} photos", response.len());

            Ok((StatusCode::OK, Json(response)))
        }
        Err(e) => {
            tracing::error!("Failed to get photos: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to get photos: {}", e),
            ))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPhotoParams {
    // PhotoId
    id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GetPhotoResponses {
    Success(PhotoViewModel),

    NotFound,
}

#[tracing::instrument(name = "Get photo", skip(app))]
pub async fn get_photo(
    State(app): State<App>,
    Path(id): Path<i32>,
) -> response::Result<impl IntoResponse, (StatusCode, String)> {
    match app.repo.get_photo(id).await {
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
