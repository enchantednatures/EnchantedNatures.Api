

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
mod api_doc;
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


