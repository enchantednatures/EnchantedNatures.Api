#![allow(dead_code)]

use std::net::SocketAddr;
use std::time::Duration;

use anyhow::Result;
use axum::error_handling::HandleErrorLayer;
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tokio::time::error::Elapsed;
use tower::BoxError;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::instrument;
use tracing::{info, subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Registry;
use utoipa::openapi::security::ApiKey;
use utoipa::openapi::security::ApiKeyValue;
use utoipa::openapi::security::SecurityScheme;
use utoipa::Modify;

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use routes::categories;
use routes::health;
use routes::photos;
use routes::photos::get_photos;
use routes::photos::post_photo;

use crate::routes::categories::{
    add_photo_to_category, categories_by_id, get::get_categories, patch::patch_category,
    put::put_category,
};
use crate::routes::health::health_check;

mod database;
mod models;
mod routes;

#[derive(OpenApi)]
#[openapi(
    paths(
        health::health_check,
        categories::categories_by_id,
        categories::get_categories,
        categories::put_category,
        categories::patch_category,
        categories::add_photo_to_category,
        photos::post_photo
    ),
    components(
        schemas(
            models::CategoryViewModel,
            models::CategoryDisplayModel,
            models::PhotoViewModel,
            models::PhotoDisplayModel,
            categories::CategoryError,
            categories::AddPhotoToCategoryRequest,
            categories::CategoryGetByIdRequest,
            categories::CreateCategoryRequest,
            categories::CategoryGetByIdResponse,
            health::HealthStatus, health::HealthStatusEnum
        ),
    ),
    modifiers(& SecurityAddon),
    tags(
        (name = "Health Checks", description = "Information about the health of the API")
    )
)]
struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("todo_apikey"))),
            )
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let formatting_layer = BunyanFormattingLayer::new("de".into(), std::io::stdout);

    let subscriber = Registry::default()
        .with(JsonStorageLayer)
        .with(formatting_layer);

    tracing::subscriber::set_global_default(subscriber).unwrap();

    let db_connection_str = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // setup connection pool
    let pool: PgPool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    sqlx::migrate!().run(&pool).await?;

    let swagger_ui = SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi());

    info!("Orphan event without a parent span");
    // build our application with some routes
    let app = Router::new()
        .merge(swagger_ui)
        .route("/health_check", get(health_check))
        .route("/api/v0/photos", get(get_photos).post(post_photo))
        .route("/api/v0/categories", get(get_categories).put(put_category))
        .route(
            "/api/v0/categories/:id",
            get(categories_by_id)
                .post(add_photo_to_category)
                .patch(patch_category),
        )
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
        .with_state(pool);

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 6969));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
