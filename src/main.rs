#![warn(dead_code)]

use api::app;
use api::auth::create_oauth_client;
use api::configuration::Settings;
use api::connect_database;
use api::database::PhotoRepository;
use api::domain::AppState;
use api::sessions::SessionManager;
use api::setup_logging;

use sqlx::PgPool;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

use utoipa_swagger_ui::{Config, SwaggerUi};

#[tokio::main]
async fn main() {
    setup_logging();
    // check_env().expect("Environment Variable must be set");

    let settings = Settings::load_config().unwrap();

    let pool: PgPool = connect_database(settings.database_settings).await;

    // let config = aws_config::from_env()
    //     .endpoint_url(&settings.aws_endpoint_url)
    //     .region(Region::new(settings.aws_region.clone().to_owned()))
    //     .load()
    //     .await;

    // let s3_client = aws_sdk_s3::Client::new(&config);
    let oauth_client = create_oauth_client(settings.auth_settings).unwrap();
    let session_manager = SessionManager::new(redis::Client::open(settings.redis_url).unwrap());

    let photo_repo = PhotoRepository::new(pool.clone());
    photo_repo.migrate().await.unwrap();
    let app_state = AppState::new(photo_repo, oauth_client, session_manager);
    let swagger_config = Config::from("/enchanted-natures.openapi.spec.yaml");
    let swagger_ui = SwaggerUi::new("/swagger-ui").config(swagger_config);
    let app = app(swagger_ui, app_state);

    let addr = SocketAddr::from((settings.app_settings.addr, settings.app_settings.port));
    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(
        listener,
        app.layer(TraceLayer::new_for_http()).into_make_service(),
    )
    .await
    .unwrap();
}
