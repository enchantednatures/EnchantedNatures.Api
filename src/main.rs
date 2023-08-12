#![allow(dead_code)]

use crate::api_doc::ApiDoc;
use crate::database::PhotoRepository;
use crate::routes::categories::{
    add_photo_to_category, categories_by_id, get_categories, post_category,
};
use crate::routes::health::health_check;
use crate::routes::photos::*;
use crate::routes::upload::save_request_body;

use aws_sdk_s3::config::Region;
use aws_sdk_s3::Client;
use axum::error_handling::HandleErrorLayer;
use axum::http::StatusCode;
use axum::routing::get;
use axum::routing::post;
use axum::Router;
use domain::AppState;
use routes::photos::get_photos;
use routes::photos::post_photo;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::error::Elapsed;
use tower::BoxError;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::Registry;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod api_doc;
mod database;
mod domain;
mod error_handling;
mod models;
mod routes;

type App = Arc<AppState>;

async fn serve(app: Router, addr: SocketAddr) {
    axum::Server::bind(&addr)
        .serve(app.layer(TraceLayer::new_for_http()).into_make_service())
        .await
        .unwrap();
}

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

    // let credentials_provider = C
    let config = aws_config::from_env()
        .endpoint_url(aws_endpoint_url)
        .region(Region::new(aws_region))
        .load()
        .await;
    let client = Client::new(&config);

    // let mut base_url = BaseUrl::from_string(aws_endpoint_url).unwrap();
    // base_url.https = false;

    // setup connection pool
    let pool: PgPool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    sqlx::migrate!().run(&pool).await.unwrap();

    let photo_repo = PhotoRepository::new(pool.clone());

    let app_state = App::new(AppState::new(photo_repo, client));

    let swagger_path = "/swagger-ui";

    let swagger_ui = SwaggerUi::new(swagger_path).url("/api-docs/openapi.json", ApiDoc::openapi());

    let app = Router::new()
        .merge(swagger_ui)
        .route("/health_check", get(health_check))
        .route("/api/v0/photos", get(get_photos).post(post_photo))
        .route(
            "/api/v0/photos/:id",
            get(get_photo).delete(delete_photo).put(put_photo),
        )
        .route(
            "/api/v0/categories",
            get(get_categories).post(post_category),
        )
        .route("/api/v0/categories/:id", get(categories_by_id))
        .route("/api/v0/categories/:id/photos", post(add_photo_to_category))
        // .route("/api/v0/upload/:file_name", post(save_request_body))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {}", error),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        )
        .with_state(app_state);

    serve(app, SocketAddr::from(([0, 0, 0, 0], 6969))).await; // s(addr, app)
}
