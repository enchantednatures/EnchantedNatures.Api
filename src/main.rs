#![allow(dead_code)]

use aws_sdk_s3::config::Region;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::SocketAddr;

use api::app::create_router;
use api::auth::create_oauth_client;
use api::configuration::ApplicationSettings;
use api::database::PhotoRepository;
use api::domain::AppState;
use api::router;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::Registry;
use utoipa_swagger_ui::{Config, SwaggerUi};

#[tokio::main]
async fn main() {
    let formatting_layer = BunyanFormattingLayer::new("enchanted_natures".into(), std::io::stdout);

    let subscriber = Registry::default()
        .with(JsonStorageLayer)
        .with(EnvFilter::new("info"))
        .with(formatting_layer);

    tracing::subscriber::set_global_default(subscriber).unwrap();

    let db_connection_str = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let aws_endpoint_url = std::env::var("AWS_ENDPOINT_URL").expect("AWS_ENDPOINT_URL must be set");
    let _aws_access_key =
        std::env::var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID must be set");
    let _aws_secret_key =
        std::env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY must be set");
    let _aws_bucket_name = std::env::var("AWS_BUCKET_NAME").expect("AWS_BUCKET_NAME must be set");
    let aws_region = std::env::var("AWS_REGION").expect("AWS_REGION must be set");

    let config = aws_config::from_env()
        .endpoint_url(aws_endpoint_url)
        .region(Region::new(aws_region))
        .load()
        .await;
    let s3_client = aws_sdk_s3::Client::new(&config);
    let oauth_client = create_oauth_client().unwrap();
    let pool: PgPool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    let photo_repo = PhotoRepository::new(pool.clone());
    photo_repo.migrate().await.unwrap();
    let app_state = AppState::new(photo_repo, oauth_client, s3_client);
    let swagger_ui = SwaggerUi::new("/swagger-ui")
        .config(Config::from("/api/enchanted-natures.openapi.spec.yaml"));
    let app = create_router(swagger_ui, app_state);
    let app_settings = ApplicationSettings::default();
    router::serve(
        app,
        SocketAddr::from((app_settings.addr, app_settings.port)),
    )
    .await;
    // s(addr, app)
}
