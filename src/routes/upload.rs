use chrono::NaiveDate;
use hyper::StatusCode;
use serde_json::json;
use std::io;
use tokio::io::AsyncReadExt;
use tracing::info;

use anyhow::Result;
use axum::{
    extract::{BodyStream, Multipart, Path, State},
    response::{self, IntoResponse},
    Json,
};
use futures::TryStreamExt;
use tokio_util::io::StreamReader;

use serde::{Deserialize, Serialize};

use crate::{auth::User, database::PhotoRepository, domain::AppState};

#[derive(Debug)]
pub enum PhotoCreateRequestBuilderError {
    TitleRequired,
    LocationTakenRequired,
    DateTakenRequired,
    FilenameRequired,
}

#[derive(Debug, Default)]
pub struct PhotoCreateRequestBuilder {
    title: String,
    location_taken: String,
    date_taken: String,
    filename: String,
}

impl PhotoCreateRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn location_taken(mut self, location_taken: String) -> Self {
        self.location_taken = location_taken;
        self
    }

    pub fn date_taken(mut self, date_taken: String) -> Self {
        self.date_taken = date_taken;
        self
    }

    pub fn filename(mut self, filename: String) -> Self {
        self.filename = filename;
        self
    }

    pub fn build(self) -> Result<PhotoCreateRequest, PhotoCreateRequestBuilderError> {
        if self.title.is_empty() {
            return Err(PhotoCreateRequestBuilderError::TitleRequired);
        }
        if self.location_taken.is_empty() {
            return Err(PhotoCreateRequestBuilderError::LocationTakenRequired);
        }
        if self.date_taken.is_empty() {
            return Err(PhotoCreateRequestBuilderError::DateTakenRequired);
        }
        if self.filename.is_empty() {
            return Err(PhotoCreateRequestBuilderError::FilenameRequired);
        }
        match self.date_taken.parse::<NaiveDate>() {
            Ok(date_taken) => Ok(PhotoCreateRequest {
                title: self.title,
                location_taken: self.location_taken,
                date_taken,
                filename: self.filename,
            }),
            Err(_) => Err(PhotoCreateRequestBuilderError::DateTakenRequired),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhotoCreateRequest {
    pub title: String,
    pub location_taken: String,
    pub date_taken: NaiveDate,
    pub filename: String,
}

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

pub async fn post_photo(
    photo_repo: PhotoRepository,
    user: User,
    payload: PhotoCreateRequest,
) -> Result<()> {
    info!("inserting photo");
    info!("{}", payload.title);
    let _photo = photo_repo
        .add_photo(
            payload.title,
            payload.filename,
            payload.location_taken,
            payload.date_taken,
        )
        .await
        .unwrap();

    info!("photo created");
    Ok(())
}

#[tracing::instrument(name = "Save file", skip(app, multipart))]
pub async fn save_request_body(
    Path(file_name): Path<String>,
    State(app): State<AppState>,
    user: User,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    tracing::info!("Uploading file for user: {:?}", user);
    let mut checksum: Option<String> = None;

    let mut title: Option<String> = None;
    let mut filename: Option<String> = None;
    let mut location_taken: Option<String> = None;
    let mut date_taken: Option<String> = None;

    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let content_type = field.content_type();
        let name = field.name().unwrap();
        match name {
            "file" => {
                filename = Some(field.file_name().unwrap().to_string());

                let mut buffer = Vec::new();

                while let Some(chunk) = field
                    .chunk()
                    .await
                    .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?
                {
                    info!("received {} bytes", chunk.len());
                    buffer.extend_from_slice(&chunk);
                }

                tracing::info!("Uploading photo {}", &file_name);
                filename = Some(file_name.to_string());

                let buffer_len = &buffer.len();
                let result = app.upload_photo(buffer, &file_name).await.unwrap();
                tracing::info!("Uploaded file: {:?} with size: {}", filename, buffer_len);
                checksum = result.checksum_crc32;
            }
            "title" => {
                title = Some(field.text().await.unwrap());
                tracing::info!("Uploaded title: {:?}", &title);
            }
            "location_taken" => {
                location_taken = Some(field.text().await.unwrap());
                tracing::info!("Uploaded location_taken: {:?}", &location_taken);
            }
            "date_taken" => {
                date_taken = Some(field.text().await.unwrap());
                tracing::info!("Uploaded date_taken: {:?}", &date_taken);
            }
            _ => {
                tracing::info!(
                    "Skipping field: {:?} with content type: {:?}",
                    name,
                    content_type
                );
            }
        }
    }

    let photo_create_request_builder = PhotoCreateRequestBuilder::new()
        .title(title.unwrap())
        .filename(filename.unwrap())
        .location_taken(location_taken.unwrap())
        .date_taken(date_taken.unwrap());

    match checksum {
        Some(checksum_crc32) => {
            let photo_create_request = photo_create_request_builder.build();
            match photo_create_request {
                Ok(photo_create_request) => {
                    post_photo(app.repo.clone(), user, photo_create_request)
                        .await
                        .unwrap();
                    Ok((StatusCode::CREATED, Json(json!({"key": checksum_crc32}))))
                }
                Err(e) => {
                    tracing::error!("Failed to upload file {}, {:?}", file_name, e);
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to upload file".into(),
                    ))
                }
            }
        }
        None => {
            tracing::error!("Failed to upload file {}", file_name);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to upload file".into(),
            ))
        }
    }
}

pub async fn add_photo_cloudflare_resource(
    State(photo_repo): State<PhotoRepository>,
    Path(photo_id): Path<i32>,
) {
    todo!()
}
