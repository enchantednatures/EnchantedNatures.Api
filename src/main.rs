#![allow(dead_code)]

use aws_sdk_s3::config::Region;
use aws_sdk_s3::Client;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::SocketAddr;

use api::app::{create_router, App};
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
    let client = Client::new(&config);

    let pool: PgPool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    sqlx::migrate!().run(&pool).await.unwrap();
    let photo_repo = PhotoRepository::new(pool.clone());
    let app_state = App::new(AppState::new(photo_repo, client));
    let swagger_ui = SwaggerUi::new("/swagger-ui")
        .config(Config::from("/api/enchanted-natures.openapi.spec.yaml"));
    let app = create_router(swagger_ui, app_state);
    router::serve(app, SocketAddr::from(([0, 0, 0, 0], 6969))).await; // s(addr, app)
}
