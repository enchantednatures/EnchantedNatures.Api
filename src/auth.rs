use anyhow::{Context, Result};

use async_session::{MemoryStore, Session, SessionStore};
use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenUrl,
};

use crate::configuration::AuthSettings;
use axum::extract::{Query, State};
use axum::headers::HeaderMap;
use axum::http::header::SET_COOKIE;
use axum::response::{IntoResponse, Redirect, Response};

use axum::http::header;
use axum::http::request::Parts;
use axum::{
    async_trait,
    extract::{rejection::TypedHeaderRejectionReason, FromRef, FromRequestParts, TypedHeader},
    headers, RequestPartsExt,
};
use serde::Deserialize;

use oauth2::{reqwest::async_http_client, AuthorizationCode, TokenResponse};
use serde::Serialize;

static COOKIE_NAME: &str = "X-Auth";

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct AuthRequest {
    code: String,
    state: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    id: String,
    avatar: Option<String>,
    username: String,
    discriminator: String,
}

pub struct AuthRedirect;

impl IntoResponse for AuthRedirect {
    fn into_response(self) -> Response {
        Redirect::temporary("/api/v0/authorize").into_response()
    }
}
#[async_trait]
impl<S> FromRequestParts<S> for User
where
    MemoryStore: FromRef<S>,
    S: Send + Sync,
{
    // If anything goes wrong or no session is found, redirect to the auth page
    type Rejection = AuthRedirect;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let store = MemoryStore::from_ref(state);

        let cookies = parts
            .extract::<TypedHeader<headers::Cookie>>()
            .await
            .map_err(|e| match *e.name() {
                header::COOKIE => match e.reason() {
                    TypedHeaderRejectionReason::Missing => AuthRedirect,
                    _ => panic!("unexpected error getting Cookie header(s): {}", e),
                },
                _ => panic!("unexpected error getting cookies: {}", e),
            })?;
        let session_cookie = cookies.get(COOKIE_NAME).ok_or(AuthRedirect)?;

        let session = store
            .load_session(session_cookie.to_string())
            .await
            .unwrap()
            .ok_or(AuthRedirect)?;

        let user = session.get::<User>("user").ok_or(AuthRedirect)?;

        Ok(user)
    }
}
pub async fn protected(user: User) -> impl IntoResponse {
    format!(
        "Welcome to the protected area :)\nHere's your info:\n{:?}",
        user
    )
}

pub async fn login_authorized(
    Query(query): Query<AuthRequest>,
    State(mut store): State<MemoryStore>,
    State(client): State<reqwest::Client>,
    State(oauth_client): State<BasicClient>,
) -> impl IntoResponse {
    let token = oauth_client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(async_http_client)
        .await
        .unwrap();

    // Fetch user data from discord
    let user_data: User = client
        // https://discord.com/developers/docs/resources/user#get-current-user
        .get("https://auth.enchantednatures.com/application/o/userinfo/")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .unwrap()
        .json::<User>()
        .await
        .unwrap();

    // Create a new session filled with user data
    let mut session = Session::new();
    session.insert("user", &user_data).unwrap();

    // Store session and get corresponding cookie
    let cookie = store.store_session(session).await.unwrap().unwrap();

    // Build the cookie
    let cookie = format!("{}={}; SameSite=Lax; Path=/", COOKIE_NAME, cookie);

    // Set cookie
    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, cookie.parse().unwrap());

    (headers, Redirect::to("/"))
}
pub async fn default_auth(State(client): State<BasicClient>) -> impl IntoResponse {
    println!("auth");
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".to_string()))
        .url();

    println!("{:?}", auth_url);
    println!("{:?}", csrf_token);
    // Redirect to Discord's oauth service
    Redirect::to(auth_url.as_ref())
}

pub fn create_oauth_client() -> Result<BasicClient> {
    // Environment variables (* = required):
    // *"CLIENT_ID"     "REPLACE_ME";
    // *"CLIENT_SECRET" "REPLACE_ME";
    //  "REDIRECT_URL"  "http://127.0.0.1:3000/auth/authorized";
    //  "AUTH_URL"      "https://discord.com/api/oauth2/authorize?response_type=code";
    //  "TOKEN_URL"     "https://discord.com/api/oauth2/token";

    let auth_settings = AuthSettings::new();
    Ok(BasicClient::new(
        ClientId::new(auth_settings.client_id),
        Some(ClientSecret::new(auth_settings.client_secret)),
        AuthUrl::new(auth_settings.auth_url)
            .context("failed to create new authorization server URL")?,
        Some(
            TokenUrl::new(auth_settings.token_url)
                .context("failed to create new token endpoint URL")?,
        ),
    )
    .set_redirect_uri(
        RedirectUrl::new(auth_settings.redirect_url)
            .context("failed to create new redirection URL")?,
    ))
}
