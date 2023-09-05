use anyhow::Result;
use axum::Router;

use api::{auth::create_oauth_client, sessions::SessionManager};
use aws_sdk_s3::config::Region;
use aws_sdk_s3::Client;

use api::app::create_router;
use api::database::PhotoRepository;
use api::domain::AppState;
use async_once::AsyncOnce;
use lazy_static::lazy_static;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::SocketAddr;
use tokio::runtime::Runtime;
use utoipa_swagger_ui::{Config, SwaggerUi};

lazy_static! {
    static ref RUNTIME: Runtime = Runtime::new().unwrap();
}

lazy_static! {
    pub static ref API: AsyncOnce<()> = AsyncOnce::new(async {
        std::env::set_var("RUN_MODE", "test");

        let app = spawn_app().await.expect("Unable to start app");
        let address = SocketAddr::from(([127, 0, 0, 1], 6969));

        tokio::spawn(async move {
            axum::Server::bind(&address)
                .serve(app.into_make_service())
                .await
                .expect("Failed to start server");
        });
    });
}

pub async fn spawn_app() -> Result<Router> {
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

    let oauth_client = create_oauth_client().unwrap();
    let redis = redis::Client::open(std::env::var("REDIS_URL").expect("REDIS_URL must be set")).unwrap();
    let session_manager = SessionManager::new(redis);
    let app_state = AppState::new(photo_repo, oauth_client, client, session_manager);

    let swagger_ui = SwaggerUi::new("/swagger-ui")
        .config(Config::from("/api/enchanted-natures.openapi.spec.yaml"));
    let app = create_router(swagger_ui, app_state);
    Ok(app)
}

pub fn use_app<F>(test: F)
where
    F: std::future::Future,
{
    RUNTIME.block_on(async move {
        API.get().await;

        test.await;
    })
}
