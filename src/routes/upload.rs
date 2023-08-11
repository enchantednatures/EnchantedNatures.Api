use hyper::StatusCode;
use serde_json::json;
use std::io;
use tokio::io::AsyncReadExt;

use axum::{
    extract::{BodyStream, Path, State},
    response::{self, IntoResponse},
    Json,
};
use futures::TryStreamExt;
use tokio_util::io::StreamReader;

use crate::App;
use serde::{Deserialize, Serialize};

use utoipa::{IntoResponses, ToResponse, ToSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema, ToResponse)]
pub struct UploadedPhotoViewModel {
    file_size: usize,
}

impl UploadedPhotoViewModel {
    pub fn new(file_size: usize) -> Self {
        Self { file_size }
    }
}

#[derive(Debug, Serialize, Deserialize, IntoResponses)]
pub enum UploadPhotoResponses {
    #[response(status = StatusCode::CREATED, description = "Get photo by id")]
    Success(UploadedPhotoViewModel),

    #[response(status = StatusCode::INTERNAL_SERVER_ERROR, description = "Unable to upload Photo")]
    UploadError,
}

#[utoipa::path(
    post,
    path = "/api/v0/upload/{file_name}",
    params(
        ("file_name"= String, Path, description = "Filename")
    ),
    request_body(content = [u8], description = "File contents", content_type = "image/jpeg")
)]
#[tracing::instrument(name = "Save file", skip(app, body))]
pub async fn save_request_body(
    State(app): State<App>,
    Path(file_name): Path<String>,
    body: BodyStream,
) -> response::Result<impl IntoResponse, (StatusCode, String)> {
    let body_with_io_error = body.map_err(|err| io::Error::new(io::ErrorKind::Other, err));
    let mut body_reader = StreamReader::new(body_with_io_error);

    let mut buffer = Vec::new();
    body_reader
        .read_to_end(&mut buffer)
        .await
        .expect("Failed to read body");

    let result = app.upload_photo(buffer, &file_name).await.unwrap();

    Ok((StatusCode::OK, Json(json!({"key": result.checksum_crc32}))))
}
