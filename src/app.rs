use crate::domain::AppState;
use crate::routes::categories::add_photo_to_category;
use crate::routes::categories::categories_by_id;
use crate::routes::categories::get_categories;
use crate::routes::categories::post_category;
use crate::routes::health::health_check;
use crate::routes::photos;
use crate::routes::photos::delete_photo;
use crate::routes::photos::get_photo;
use crate::routes::photos::put_photo;
use axum::error_handling::HandleErrorLayer;
use axum::http::Method;
use axum::http::StatusCode;
use axum::routing::get;
use axum::routing::post;
use axum::Router;
use photos::get_photos;
use photos::post_photo;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::error::Elapsed;
use tower::BoxError;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use utoipa_swagger_ui::SwaggerUi;

use tower_http::services::ServeFile;

pub type App = Arc<AppState>;

pub fn create_router(swagger_ui: SwaggerUi, app_state: App) -> Router {
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    Router::new()
        .merge(swagger_ui)
        .nest_service(
            "/api/enchanted-natures.openapi.spec.yaml",
            ServeFile::new("api/enchanted-natures.openapi.spec.yaml"),
        )
        .route("/health_check", get(health_check))
        .nest(
            "/api/v0",
            Router::new()
                .route("/photos", get(get_photos).post(post_photo))
                .route(
                    "/photos/:id",
                    get(get_photo).delete(delete_photo).put(put_photo),
                )
                .route("/categories", get(get_categories).post(post_category))
                .route("/categories/:id", get(categories_by_id))
                .route("/categories/:id/photos", post(add_photo_to_category)),
        )
        .layer(cors)
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
        .with_state(app_state)
}
