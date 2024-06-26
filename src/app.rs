#![warn(dead_code)]

use tower_http::trace::TraceLayer;

use utoipa_swagger_ui::{SwaggerUi};

use axum::error_handling::HandleErrorLayer;
use axum::extract::MatchedPath;
use axum::http::Method;
use axum::http::StatusCode;
use axum::response::Response;
use axum::routing::get;
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

use crate::auth::default_auth;
use crate::auth::login_authorized;
use crate::domain::AppState;
use crate::routes::categories_router;
use crate::routes::health_check;
use crate::routes::photo_router;

pub fn app(swagger_ui: SwaggerUi, app_state: AppState) -> Router {
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    Router::new()
        .merge(swagger_ui)
        .nest_service(
            "/enchanted-natures.openapi.spec.yaml",
            ServeFile::new("specs/enchanted-natures.openapi.spec.yaml"),
        )
        .route("/authorize", get(default_auth))
        .route("/authorized", get(login_authorized))
        .route("/health_check", get(health_check))
        .nest(
            "/api/v0",
            Router::new()
                .merge(photo_router())
                .merge(categories_router()),
        )
        .layer(
            ServiceBuilder::new()
                .layer(cors)
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
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(|request: &Request<_>| {
                            // Log the matched route's path (with placeholders not filled in).
                            // Use request.uri() or OriginalUri if you want the real path.
                            let matched_path = request
                                .extensions()
                                .get::<MatchedPath>()
                                .map(MatchedPath::as_str);

                            info_span!(
                                "http_request",
                                method = ?request.method(),
                                matched_path,
                                some_other_field = tracing::field::Empty,
                            )
                        })
                        .on_request(|_request: &Request<_>, _span: &Span| {
                            // You can use `_span.record("some_other_field", value)` in one of these
                            // closures to attach a value to the initially empty field in the info_span
                            // created above.
                        })
                        .on_response(|_response: &Response, _latency: Duration, _span: &Span| {
                            // ...
                        })
                        .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {
                            // ...
                        })
                        .on_eos(
                            |_trailers: Option<&HeaderMap>,
                             _stream_duration: Duration,
                             _span: &Span| {
                                // ...
                            },
                        )
                        .on_failure(
                            |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                                // ...
                            },
                        ),
                )
                .into_inner(),
        )
        .with_state(app_state)
}
