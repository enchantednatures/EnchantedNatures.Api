use crate::database::{PhotoRepo, PhotoRepository};
use crate::models::{Photo, PhotoViewModel};
use crate::{App};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{response, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::{IntoParams, IntoResponses, ToSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PhotoGetAllResponse;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PhotoCreateRequest {
    pub name: String,
    pub description: String,
    pub url: String,
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
pub async fn post_photo(
    State(app): State<App>,
    Json(payload): Json<PhotoCreateRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let photo = app
        .repo
        .add_photo(payload.name, payload.description, payload.url)
        .await
        .unwrap();

    Ok((StatusCode::CREATED, Json(photo)))
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
    match app.repo.get_photos().await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
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

// async fn stream_to_file<S, E>(path: &str, stream: S) -> Result<(), (StatusCode, String)>
// where
//     S: Stream<Item = Result<Bytes, E>>,
//     E: Into<BoxError>,
// {
//     if !path_is_valid(path) {
//         return Err((StatusCode::BAD_REQUEST, "Invalid path".to_owned()));
//     }

//     async {
//         // Convert the stream into an `AsyncRead`.
//         let body_with_io_error = stream.map_err(|err| io::Error::new(io::ErrorKind::Other, err));
//         let body_reader = StreamReader::new(body_with_io_error);
//         futures::pin_mut!(body_reader);

//         // Create the file. `File` implements `AsyncWrite`.
//         let path = std::path::Path::new(UPLOADS_DIRECTORY).join(path);
//         let mut file = BufWriter::new(File::create(path).await?);

//         // Copy the body into the file.
//         tokio::io::copy(&mut body_reader, &mut file).await?;

//         Ok::<_, io::Error>(())
//     }
//     .await
//     .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
// }

pub struct AppState {
    pub repo: PhotoRepository,
    pub client: aws_sdk_s3::Client,
}

impl AppState {
    pub fn new(repo: PhotoRepository, client: aws_sdk_s3::Client) -> Self {
        Self { repo, client }
    }
}
// pub async fn upload_photo(State(app): State(AppState)) {}
