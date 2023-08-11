use crate::database::PhotoRepo;
use crate::models::{Photo, PhotoViewModel};
use crate::App;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{response, Json};
use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::info;
use utoipa::{IntoParams, IntoResponses, ToSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PhotoGetAllResponse;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PhotoCreateRequest {
    pub title: String,
    pub location_taken: String,
    pub date_taken: NaiveDate,
    pub description: String,
    pub filename: String,
}

#[utoipa::path(
    delete,
    path = "/api/v0/photos/{id}",
    params(
        ("id"= i32, Path, description = "Id of the Photo")
    ),
    responses(
        (status = StatusCode::NO_CONTENT, description = "Delete photo with Id"),
    )
)]
pub async fn delete_photo(
    State(app): State<App>,
    Path(id): Path<i32>,
) -> response::Result<impl IntoResponse, (StatusCode, String)> {
    let result = app.repo.delete_photo(id).await;

    match result {
        Ok(_) => Ok((StatusCode::NO_CONTENT, Json(json!({ "deleted": &id })))),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to delete photo: {}", err),
        )),
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PhotoCreatedResponse {
    pub photo: Photo,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoResponses)]
#[response(description = "Delete a category", content_type = "application/json")]
pub enum CreatePhotoResponses {
    #[response(status = StatusCode::CREATED, description = "Photo Created")]
    Created(Photo),

    #[response(status = StatusCode::CONFLICT, description = "Server Error")]
    AlreadyExist,

    #[response(status = StatusCode::INTERNAL_SERVER_ERROR, description = "Server Error")]
    BadRequest,
}

#[utoipa::path(
    post,
    path = "/api/v0/photos",
    request_body = PhotoCreateRequest,
    responses(
        CreatePhotoResponses
    )
)]
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
            payload.description,
            payload.filename,
            payload.location_taken,
            payload.date_taken,
        )
        .await
        .unwrap();

    info!("photo created");
    Ok((StatusCode::CREATED, Json(photo)))
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PhotoUpdateRequest { 
    pub title: Option<String>,
    pub location_taken: Option<String>,
    pub date_taken: Option<NaiveDate>,
    pub description: Option<String>,
    pub filename: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoResponses)]
#[response(description = "Update a photo", content_type = "application/json")]
pub enum UpdatePhotoResponses {
    #[response(status = StatusCode::OK, description = "Photo Updated")]
    Updated(Photo),

    #[response(status = StatusCode::NOT_FOUND, description = "Photo does not exist")]
    DoesNotExist,

    #[response(status = StatusCode::INTERNAL_SERVER_ERROR, description = "Server Error")]
    BadRequest,
}

#[utoipa::path(
    put,
    path = "/api/v0/photos/{id}",
    request_body = PhotoUpdateRequest,
    responses(
        UpdatePhotoResponses
    )
)]
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
            payload.description,
            payload.filename,
            payload.location_taken,
            payload.date_taken,
        )
        .await
        .unwrap();
    info!("photo Updated");
    Ok((StatusCode::OK, Json(photo)))
}



#[derive(Debug, Serialize, Deserialize, IntoResponses)]
pub enum GetPhotosResponses {
    #[response(status = StatusCode::OK, description = "Get all photos")]
    Success(Vec<Photo>),
}

#[utoipa::path(get, path = "/api/v0/photos", responses(GetPhotosResponses))]
#[tracing::instrument(name = "Get photos", skip(app))]
pub async fn get_photos(
    State(app): State<App>,
) -> response::Result<impl IntoResponse, (StatusCode, String)> {
    info!("getting all photos");
    match app.repo.get_photos().await {
        Ok(response) => {
            info!("retrieved {} photos", response.len());

            Ok((StatusCode::OK, Json(response)))
        },
        Err(e) => {
            tracing::error!("Failed to get photos: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to get photos: {}", e),
            ))
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
#[into_params(parameter_in = Path)]
pub struct GetPhotoParams {
    // PhotoId
    id: i32,
}

#[derive(Debug, Serialize, Deserialize, IntoResponses)]
pub enum GetPhotoResponses {
    #[response(status = StatusCode::OK, description = "Get photo by id")]
    Success(PhotoViewModel),

    #[response(status = StatusCode::NOT_FOUND, description = "Unable to find Photo")]
    NotFound,
}

#[utoipa::path(
    get,
    path = "/api/v0/photos/{id}",
    params(
        ("id"= i32, Path, description = "Photo Id")
    ),
    responses(GetPhotoResponses)
)]
#[tracing::instrument(name = "Get photos", skip(app))]
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
