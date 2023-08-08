#![allow(dead_code)]

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;

use aws_sdk_s3::config::Region;
use aws_sdk_s3::Client;
use axum::error_handling::HandleErrorLayer;
use axum::http::StatusCode;
use axum::routing::get;
use axum::routing::post;
use axum::Router;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tokio::time::error::Elapsed;
use tower::BoxError;
use tower::ServiceBuilder;

use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use routes::categories;
use routes::health;
use routes::photos;
use routes::photos::get_photos;
use routes::photos::post_photo;

use crate::database::PhotoRepository;
use crate::routes::categories::{
    add_photo_to_category, categories_by_id, get_categories, post_category, put_category,
};
use crate::routes::health::health_check;
use crate::routes::photos::*;

mod database;
mod domain;
mod models;
mod routes;

#[derive(OpenApi)]
#[openapi(
    paths(
        health::health_check,
        categories::categories_by_id,
        categories::get_categories,
        categories::put_category,
        categories::post_category,
        categories::add_photo_to_category,
        photos::post_photo,
        photos::get_photo,
        photos::get_photos,
        photos::delete_photo
    ),
    components(
        schemas(
            models::CategoryViewModel,
            models::CategoryDisplayModel,
            models::PhotoViewModel,
            models::PhotoDisplayModel,
            models::Photo,
            models::Category,
            photos::PhotoCreateRequest,
            categories::CategoryError,
            categories::AddPhotoToCategoryRequest,
            categories::PatchCategoryRequestBody,
            categories::UpdatePhotoCategoryRequest,
            categories::UpdatePhotoCategoryResponse,
            categories::CategoryGetByIdRequest,
            categories::CreateCategoryRequest,
            categories::CategoryGetByIdResponse,
            health::HealthStatus, health::HealthStatusEnum
        ),
    ),
    tags(
        (name = "Health Checks", description = "Information about the health of the API")
    )
)]
struct ApiDoc;

type App = Arc<AppState>;

#[tokio::main]
async fn main() -> Result<()> {
    let formatting_layer = BunyanFormattingLayer::new("enchanted_natures".into(), std::io::stdout);

    let subscriber = Registry::default()
        .with(JsonStorageLayer)
        .with(formatting_layer);

    tracing::subscriber::set_global_default(subscriber).unwrap();

    let db_connection_str = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let aws_endpoint_url = std::env::var("AWS_ENDPOINT_URL").expect("AWS_ENDPOINT_URL must be set");
    let _aws_access_key =
        std::env::var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID must be set");
    let _aws_secret_key =
        std::env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY must be set");
    let _aws_bucket_name = std::env::var("AWS_BUCKET_NAME").expect("AWS_BUCKET_NAME must be set");
    let _aws_region = std::env::var("AWS_REGION").expect("AWS_REGION must be set");

    // let credentials_provider = C
    let config = aws_config::from_env()
        .endpoint_url(aws_endpoint_url)
        .region(Region::new("us-east-rack-01"))
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

    sqlx::migrate!().run(&pool).await?;

    let photo_repo = PhotoRepository::new(pool.clone());

    let app_state = App::new(AppState::new(photo_repo, client));

    let swagger_path = "/swagger-ui";

    let swagger_ui = SwaggerUi::new(swagger_path).url("/api-docs/openapi.json", ApiDoc::openapi());

    // build our application with some routes
    let app = Router::new()
        .merge(swagger_ui)
        .route("/health_check", get(health_check))
        .route("/api/v0/photos", get(get_photos).post(post_photo))
        .route("/api/v0/photos/:id", get(get_photo).delete(delete_photo))
        .route("/api/v0/categories", get(get_categories).put(put_category))
        .route(
            "/api/v0/categories/:id",
            get(categories_by_id).post(post_category),
        )
        .route("/api/v0/categories/:id/photos", post(add_photo_to_category))
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

    // run it with hyper
    let addr = SocketAddr::from(([0, 0, 0, 0], 6969));

    let msg = format!("Starting server at http://{}/{}", addr, swagger_path);
    info!(msg);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
