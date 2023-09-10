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

use serde::{Deserialize, Serialize};

use crate::{auth::User, domain::AppState};

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadedPhotoViewModel {
    file_size: usize,
}

impl UploadedPhotoViewModel {
    pub fn new(file_size: usize) -> Self {
        Self { file_size }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UploadPhotoResponses {
    Success(UploadedPhotoViewModel),
    UploadError,
}

// #[tracing::instrument(name = "Save file", skip(app, body))]
pub async fn save_request_body(
    Path(file_name): Path<String>,
    State(app): State<AppState>,
    user: User,
    body: BodyStream,
) -> response::Result<impl IntoResponse, (StatusCode, String)> {
    tracing::info!("Uploading file for user: {:?}", user);
    let body_with_io_error = body.map_err(|err| io::Error::new(io::ErrorKind::Other, err));
    let mut body_reader = StreamReader::new(body_with_io_error);

    let mut buffer = Vec::new();
    body_reader
        .read_to_end(&mut buffer)
        .await
        .expect("Failed to read body");

    tracing::info!("Uploading photo {}", &file_name);
    let result = app.upload_photo(buffer, &file_name).await.unwrap();

    Ok((StatusCode::OK, Json(json!({"key": result.checksum_crc32}))))
}
