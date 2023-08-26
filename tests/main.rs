use aws_sdk_s3::config::Region;
use aws_sdk_s3::Client;
use std::net::{SocketAddr, TcpListener};

use api::app::{create_router, App};
use api::database::PhotoRepository;
use api::domain::AppState;
use axum::http::Request;
use axum::{
    extract::ConnectInfo,
    routing::{get, post},
    Json, Router,
};
use hyper::Body;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa_swagger_ui::{Config, SwaggerUi};

#[tokio::test]
async fn the_real_deal() {
    let listener = TcpListener::bind("0.0.0.0:0".parse::<SocketAddr>().unwrap()).unwrap();
    let addr = listener.local_addr().unwrap();

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
    tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(app.into_make_service())
            .await
            .unwrap();
    });

    let client = hyper::Client::new();

    let response = client
        .request(
            Request::builder()
                .uri(format!("http://{}/health_check", addr))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    assert_eq!(&body[..], b"{\"status\":\"Ok\"}");
}
