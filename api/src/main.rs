#![warn(dead_code)]

use anyhow::Result;
use api::auth::create_oauth_client;
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

use api::auth::{default_auth, login_authorized};

use api::routes::health::health_check;

use api::routes::categories_router;
use api::routes::photos::photo_router;
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

pub fn create_router(swagger_ui: SwaggerUi, app_state: AppState) -> Router {
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

fn setup_logging() {
    let formatting_layer = BunyanFormattingLayer::new("enchanted_natures".into(), std::io::stdout);
    let subscriber = Registry::default()
        .with(JsonStorageLayer)
        .with(EnvFilter::new("info"))
        .with(formatting_layer);

    tracing::subscriber::set_global_default(subscriber).unwrap();
}

async fn connect_database(database_url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("can't connect to database")
}

fn check_env() -> Result<()> {
    let _access_key_id = std::env::var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID must be set");
    let _aws_secret_key =
        std::env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY must be set");
    Ok(())
}

#[tokio::main(worker_threads = 16)]
async fn main() {
    setup_logging();
    check_env().expect("Environment Variable must be set");

    let settings = Settings::load_config().unwrap();

    let pool: PgPool = connect_database(settings.database_url.as_str()).await;

    let config = aws_config::from_env()
        .endpoint_url(&settings.aws_endpoint_url)
        .region(Region::new(settings.aws_region.clone().to_owned()))
        .load()
        .await;

    let s3_client = aws_sdk_s3::Client::new(&config);
    let oauth_client = create_oauth_client(settings.auth_settings).unwrap();
    let session_manager = SessionManager::new(redis::Client::open(settings.redis_url).unwrap());

    let photo_repo = PhotoRepository::new(pool.clone());
    photo_repo.migrate().await.unwrap();
    let app_state = AppState::new(photo_repo, oauth_client, s3_client, session_manager);
    let swagger_config = Config::from("/enchanted-natures.openapi.spec.yaml");
    let swagger_ui = SwaggerUi::new("/swagger-ui").config(swagger_config);
    let app = create_router(swagger_ui, app_state);

    let addr = SocketAddr::from((settings.app_settings.addr, settings.app_settings.port));

    Server::bind(&addr)
        .serve(app.layer(TraceLayer::new_for_http()).into_make_service())
        .await
        .unwrap();
}
