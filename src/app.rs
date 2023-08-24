use crate::auth::{default_auth, login_authorized};
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
use axum::http::StatusCode;
use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use axum::routing::get;
use axum::routing::post;
use axum::Router;
use photos::get_photos;
use photos::post_photo;
use std::time::Duration;
use tokio::time::error::Elapsed;
use tower::BoxError;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use utoipa_swagger_ui::SwaggerUi;

pub fn create_router(swagger_ui: SwaggerUi, app_state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(
            "https://enchantednatures.com"
                .parse::<HeaderValue>()
                .unwrap(),
        )
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    Router::new()
        .merge(swagger_ui)
        .route("/api/v0/authorize", get(default_auth))
        .route("/api/v0/authorized", get(login_authorized))
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
        .layer(cors)
        .with_state(app_state)
}
