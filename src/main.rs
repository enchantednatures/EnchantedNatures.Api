#![warn(dead_code)]

use anyhow::Result;
use api::app::create_router;
use api::configuration::Settings;
use api::database::PhotoRepository;
use api::domain::AppState;
use api::sessions::SessionManager;
use aws_sdk_s3::config::Region;

use axum::Server;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::Registry;
use utoipa_swagger_ui::{Config, SwaggerUi};

use api::routes::health::health_check;

use api::routes::categories_router;
use api::routes::photos::photo_router;
use axum::error_handling::HandleErrorLayer;
use axum::extract::MatchedPath;
use axum::http::Method;
use axum::http::StatusCode;
use axum::response::Response;
use axum::routing::get;
use axum::routing::post;
use axum::Router;
use hyper::body::Bytes;
use hyper::{HeaderMap, Request};

use std::time::Duration;
use tokio::time::error::Elapsed;
use tower::BoxError;
use tower::ServiceBuilder;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::cors::{Any, CorsLayer};
use tracing::{info_span, Span};

use tower_http::services::ServeFile;


fn setup_logging() {
    let formatting_layer = BunyanFormattingLayer::new("enchanted_natures".into(), std::io::stdout);
    let subscriber = Registry::default()
        .with(JsonStorageLayer)
        .with(EnvFilter::new("info"))
        .with(formatting_layer);

    tracing::subscriber::set_global_default(subscriber).unwrap();
}

#[shuttle_runtime::main]
async fn shuttle(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    setup_logging();
    let photo_repo = PhotoRepository::new(pool.clone());
    photo_repo.migrate().await.unwrap();
    let app_state = AppState::new(photo_repo);
    let swagger_config = Config::from("/enchanted-natures.openapi.spec.yaml");
    let swagger_ui = SwaggerUi::new("/swagger-ui").config(swagger_config);
    let app = create_router(swagger_ui, app_state);
    Ok(app.into())
}
