use anyhow::{Context, Result};

use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenUrl,
};

use crate::configuration::AuthSettings;
use axum::extract::{Query, State};
use axum::headers::HeaderMap;
use axum::http::header::SET_COOKIE;
use axum::response::{IntoResponse, Redirect};
use serde::Deserialize;

use crate::sessions::SessionManager;
use oauth2::{reqwest::async_http_client, AuthorizationCode, TokenResponse};
use serde::Serialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct AuthRequest {
    code: String,
    state: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct User {
    id: String,
    avatar: Option<String>,
    username: String,
    discriminator: String,
}
pub async fn login_authorized(
    Query(query): Query<AuthRequest>,
    State(mut session): State<SessionManager>,
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
        .get("https://discordapp.com/api/users/@me")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .unwrap()
        .json::<User>()
        .await
        .unwrap();

    session.insert("user", &user_data).unwrap();

    let cookie = format!("{}={}; SameSite=Lax; Path=/", "EXAMPLE", "");
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
