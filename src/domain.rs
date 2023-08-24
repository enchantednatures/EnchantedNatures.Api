use crate::database::PhotoRepository;
use crate::sessions::SessionManager;
use anyhow::{Ok, Result};
use aws_sdk_s3::{operation::put_object::PutObjectOutput, primitives::ByteStream};
use axum::extract::FromRef;
use oauth2::basic::BasicClient;
use reqwest;

#[derive(Debug, Clone)]
pub struct AppState {
    pub repo: PhotoRepository,
    pub s3_client: aws_sdk_s3::Client,
    pub http_client: reqwest::Client,
    pub oauth_client: BasicClient,
    pub session_manager: SessionManager,
    bucket_name: String,
}

impl AppState {
    pub fn new(
        repo: PhotoRepository,
        oauth_client: BasicClient,
        s3_client: aws_sdk_s3::Client,
    ) -> Self {
        Self {
            repo,
            s3_client,
            http_client: reqwest::Client::new(),
            oauth_client,
            session_manager: SessionManager::new(),
            bucket_name: "photos".into(),
        }
    }

    pub async fn upload_photo(
        &self,
        body: Vec<u8>,
        key: &str,
    ) -> Result<PutObjectOutput, anyhow::Error> {
        let result = self
            .s3_client
            .put_object()
            .bucket(&self.bucket_name)
            .key(key)
            .body(ByteStream::from(body))
            .send()
            .await?;

        Ok(result)
    }
}

impl FromRef<AppState> for PhotoRepository {
    fn from_ref(state: &AppState) -> Self {
        state.repo.clone()
    }
}

impl FromRef<AppState> for SessionManager {
    fn from_ref(state: &AppState) -> Self {
        state.session_manager.clone()
    }
}

impl FromRef<AppState> for aws_sdk_s3::Client {
    fn from_ref(state: &AppState) -> Self {
        state.s3_client.clone()
    }
}

impl FromRef<AppState> for reqwest::Client {
    fn from_ref(state: &AppState) -> Self {
        state.http_client.clone()
    }
}

impl FromRef<AppState> for BasicClient {
    fn from_ref(state: &AppState) -> Self {
        state.oauth_client.clone()
    }
}
