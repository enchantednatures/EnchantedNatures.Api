use api::auth::create_oauth_client;
use api::configuration::Settings;
use api::connect_database;
use api::database::PhotoRepository;
use api::domain::AppState;
use api::sessions::SessionManager;
use api::{app,  setup_logging};

use sqlx::PgPool;

use utoipa_swagger_ui::{Config, SwaggerUi};

use axum::http::StatusCode;
use hyper::Request;

use axum::{
    body::Body,
    http::{self},
};
use tower::ServiceExt;

#[tokio::test]
async fn default() {
    setup_logging();
    // check_env().expect("Environment Variable must be set");

    let settings = Settings::load_config().unwrap();

    let pool: PgPool = connect_database(settings.database_settings).await;

    let oauth_client = create_oauth_client(settings.auth_settings).unwrap();
    let session_manager = SessionManager::new(redis::Client::open(settings.redis_url).unwrap());

    let photo_repo = PhotoRepository::new(pool.clone());
    photo_repo.migrate().await.unwrap();
    let app_state = AppState::new(photo_repo, oauth_client, session_manager);
    let swagger_config = Config::from("/enchanted-natures.openapi.spec.yaml");
    let swagger_ui = SwaggerUi::new("/swagger-ui").config(swagger_config);
    let app = app(swagger_ui, app_state);
    let request = Request::builder()
        .uri("/health_check")
        .method(http::Method::GET)
        .body(Body::empty())
        .unwrap();

    let resp = app.oneshot(request).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    // Arrange
}
