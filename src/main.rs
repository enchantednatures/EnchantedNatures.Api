#![warn(dead_code)]

use anyhow::Result;
use sessions::SessionManager;
use app::create_router;
use aws_sdk_s3::config::Region;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::SocketAddr;

use auth::create_oauth_client;
use configuration::Settings;
use database::PhotoRepository;
use domain::AppState;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::Registry;
use utoipa_swagger_ui::{Config, SwaggerUi};


pub mod app;
pub mod auth;
pub mod configuration;
pub mod database;
pub mod domain;
pub mod error_handling;
pub mod models;
pub mod routes;
pub mod router;
pub mod sessions;

fn setup_logging() {
    let formatting_layer = BunyanFormattingLayer::new("enchanted_natures".into(), std::io::stdout);
    let subscriber = Registry::default()
        .with(JsonStorageLayer)
        .with(EnvFilter::new("info"))
        .with(formatting_layer);

    tracing::subscriber::set_global_default(subscriber).unwrap();
}

async fn connect_database(database_url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("can't connect to database")
}

fn check_env() -> Result<()> { 
    let _access_key_id = std::env::var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID must be set");
    let _aws_secret_key =
        std::env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY must be set");
    Ok(())
}

#[tokio::main]
async fn main() {
    setup_logging();
    check_env().expect("Environment Variable must be set");

    let settings = Settings::load_config().unwrap();
    
    let pool: PgPool = connect_database(&settings.database_url.as_str()).await;

    let config = aws_config::from_env()
        .endpoint_url(&settings.aws_endpoint_url)
        .region(Region::new(settings.aws_region.clone().to_owned()))
        .load()
        .await;

    let s3_client = aws_sdk_s3::Client::new(&config);
    let oauth_client = create_oauth_client(settings.auth_settings).unwrap();
    let session_manager = SessionManager::new(
        redis::Client::open(std::env::var("REDIS_URL").expect("REDIS_URL must be set")).unwrap(),
    );

    let photo_repo = PhotoRepository::new(pool.clone());
    photo_repo.migrate().await.unwrap();
    let app_state = AppState::new(photo_repo, oauth_client, s3_client, session_manager);
    let swagger_config = Config::from("/enchanted-natures.openapi.spec.yaml");
    let swagger_ui = SwaggerUi::new("/swagger-ui").config(swagger_config);
    let app = create_router(swagger_ui, app_state);
    router::serve(
        app,
        SocketAddr::from((settings.app_settings.addr, settings.app_settings.port)),
    )
    .await;
    // s(addr, app)
}
