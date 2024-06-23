use api::auth::create_oauth_client;
use api::configuration::Settings;
use api::connect_database;
use api::database::PhotoRepository;
use api::domain::AppState;
use api::models::{Photo, PhotoViewModel};
use api::sessions::SessionManager;
use api::{app, check_env, setup_logging};
use aws_sdk_s3::config::Region;

use axum::{response, Router};
use sqlx::{PgPool, Value};

use utoipa_swagger_ui::{Config, SwaggerUi};

use axum::http::StatusCode;
use hyper::{Request, Response};

use axum::{
    body::Body,
    http::{self},
};
use tower::ServiceExt;

async fn test_app() -> Router {
    setup_logging();
    check_env().expect("Environment Variable must be set");

    let settings = Settings::load_config().unwrap();

    let pool: PgPool = connect_database(settings.database_url.as_str()).await;

    let config = aws_config::from_env()
        .endpoint_url(&settings.aws_endpoint_url)
        .region(Region::new(settings.aws_region.clone().to_owned()))
        .load()
        .await;

    let s3_client = aws_sdk_s3::Client::new(&config);
    let oauth_client = create_oauth_client(settings.auth_settings).unwrap();
    let session_manager = SessionManager::new(redis::Client::open(settings.redis_url).unwrap());

    let photo_repo = PhotoRepository::new(pool.clone());
    photo_repo.migrate().await.unwrap();
    let app_state = AppState::new(photo_repo, oauth_client, s3_client, session_manager);
    let swagger_config = Config::from("/enchanted-natures.openapi.spec.yaml");
    let swagger_ui = SwaggerUi::new("/swagger-ui").config(swagger_config);

    app(swagger_ui, app_state)
}

#[tokio::test]
async fn default() {
    let app = test_app().await;

    let request = Request::builder()
        .uri("/health_check")
        .method(http::Method::GET)
        .body(Body::empty())
        .unwrap();

    let resp = app.oneshot(request).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn get_all_photos() {
    let app = test_app().await;

    let request = Request::builder()
        .uri("/api/v0/photos")
        .method(http::Method::GET)
        .body(Body::empty())
        .unwrap();

    let resp = app.oneshot(request).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = hyper::body::to_bytes(resp.into_body()).await.unwrap();
    let body: Vec<PhotoViewModel> = serde_json::from_slice(&body).unwrap();

    assert!(!body.is_empty());
    // let body: = serde_json::from_str(resp.body()().await.unwrap().as_str())
}

#[tokio::test]
async fn not_found() {
    let app = test_app().await;
    let request = Request::builder()
        .uri("/not-a-valid-path")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    assert!(body.is_empty());
}
