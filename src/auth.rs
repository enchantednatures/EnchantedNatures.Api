use anyhow::{Context, Result};

use async_session::Session;
use axum_extra::typed_header::TypedHeaderRejectionReason;
use axum_extra::{headers, TypedHeader};
use hyper::HeaderMap;
use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenUrl,
};
use oauth2::{IntrospectionUrl, RevocationUrl};

use crate::configuration::AuthSettings;
use crate::sessions::SessionManager;
use axum::extract::{Query, State};
use axum::http::header::SET_COOKIE;
use axum::response::{IntoResponse, Redirect, Response};

use axum::http::header;
use axum::http::request::Parts;
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    RequestPartsExt,
};
use serde::Deserialize;

use oauth2::{reqwest::async_http_client, AuthorizationCode, TokenResponse};
use serde::Serialize;

static COOKIE_NAME: &str = "SESSION";

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    code: String,
    state: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub email: String,
    pub sub: String,
}

pub struct AuthRedirect;

impl IntoResponse for AuthRedirect {
    fn into_response(self) -> Response {
        Redirect::temporary("/authorize").into_response()
    }
}
#[async_trait]
impl<S> FromRequestParts<S> for User
where
    SessionManager: FromRef<S>,
    S: Send + Sync,
{
    // If anything goes wrong or no session is found, redirect to the auth page
    type Rejection = AuthRedirect;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let store = SessionManager::from_ref(state);

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
            .get_session(session_cookie)
            .await
            .unwrap()
            .ok_or(AuthRedirect)?;

        let user = session.get::<User>("user").ok_or(AuthRedirect)?;

        Ok(user)
    }
}

#[tracing::instrument(name = "Protected area")]
pub async fn protected(user: User) -> impl IntoResponse {
    format!(
        "Welcome to the protected area :)\nHere's your info:\n{:?}",
        user
    )
}

#[tracing::instrument(name = "Login authorized", skip(store, oauth_client))]
pub async fn login_authorized(
    Query(query): Query<AuthRequest>,
    State(store): State<SessionManager>,
    State(oauth_client): State<BasicClient>,
) -> impl IntoResponse {
    let AuthRequest {
        code,
        state: _state,
    } = query;

    let token = oauth_client
        .exchange_code(AuthorizationCode::new(code))
        .request_async(async_http_client)
        .await
        .unwrap();

    let access_token_secret = token.access_token().secret();

    let client = reqwest::Client::new();
    let user_data: User = client
        .get(oauth_client.introspection_url().unwrap().url().as_str())
        .bearer_auth(access_token_secret)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    // Create a new session filled with user data
    let mut session = Session::new();
    session.insert("user", &user_data).unwrap();

    // Store session and get corresponding cookie
    let cookie = store.set_session(&session).await.unwrap();

    // Build the cookie
    let cookie = format!("{}={}; SameSite=Lax; Path=/", COOKIE_NAME, cookie);

    // Set cookie
    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, cookie.parse().unwrap());

    (headers, Redirect::to("/swagger-ui"))
}

#[tracing::instrument(name = "Default Auth", skip(client))]
pub async fn default_auth(State(client): State<BasicClient>) -> impl IntoResponse {
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("openid".to_string()))
        .url();

    Redirect::to(auth_url.as_ref())
}

pub fn create_oauth_client(auth_settings: AuthSettings) -> Result<BasicClient> {
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
    .set_revocation_uri(RevocationUrl::new(auth_settings.revocation_url)?)
    .set_introspection_uri(IntrospectionUrl::new(auth_settings.introspection_url)?)
    .set_redirect_uri(
        RedirectUrl::new(auth_settings.redirect_url)
            .context("failed to create new redirection URL")?,
    ))
}
