use crate::{database::PhotoRepository, sessions::SessionManager};
use anyhow::{Ok, Result};

use axum::extract::FromRef;
use oauth2::basic::BasicClient;
use reqwest;

#[derive(Debug, Clone)]
pub struct AppState {
    pub repo: PhotoRepository,
    pub http_client: reqwest::Client,
    pub oauth_client: BasicClient,
    session_store: SessionManager,
    bucket_name: String,
}

impl AppState {
    pub fn new(
        repo: PhotoRepository,
        oauth_client: BasicClient,
        session_store: SessionManager,
    ) -> Self {
        Self {
            repo,
            http_client: reqwest::Client::new(),
            oauth_client,
            bucket_name: "photos".into(),
            session_store,
        }
    }
}

impl FromRef<AppState> for PhotoRepository {
    fn from_ref(state: &AppState) -> Self {
        state.repo.clone()
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

impl FromRef<AppState> for SessionManager {
    fn from_ref(state: &AppState) -> Self {
        state.session_store.clone()
    }
}
