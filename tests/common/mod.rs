use api::app::create_router;
use api::auth::create_oauth_client;
use api::configuration::Settings;
use api::database::PhotoRepository;
use api::domain::AppState;
use api::sessions::SessionManager;
use aws_sdk_s3::config::Region;
use aws_sdk_s3::Client;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use utoipa_swagger_ui::{Config, SwaggerUi};

use std::net::SocketAddr;

use hyper::server::Server;

use tokio::sync::OnceCell;

static SERVER: OnceCell<SocketAddr> = OnceCell::const_new();

pub(crate) async fn get_or_init_server() -> &'static SocketAddr {
    SERVER.get_or_init(spawn_server).await
}

async fn spawn_server() -> SocketAddr {
    let settings = Settings::load_config().unwrap();

    let _aws_access_key =
        std::env::var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID must be set");
    let _aws_secret_key =
        std::env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY must be set");

    let config = aws_config::from_env()
        .endpoint_url(&settings.aws_endpoint_url)
        .region(Region::new(settings.aws_region.clone().to_owned()))
        .load()
        .await;

    let client = Client::new(&config);
    let pool: PgPool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&settings.database_url)
        .await
        .expect("can't connect to database");

    let photo_repo = PhotoRepository::new(pool.clone());

    photo_repo.migrate().await.unwrap();

    let oauth_client = create_oauth_client(settings.auth_settings).unwrap();

    let session_manager = SessionManager::new(redis::Client::open(settings.redis_url).unwrap());
    let app_state = AppState::new(photo_repo, oauth_client, client, session_manager);

    let swagger_ui = SwaggerUi::new("/swagger-ui")
        .config(Config::from("/api/enchanted-natures.openapi.spec.yaml"));
    let app = create_router(swagger_ui, app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 0));

    let server = Server::bind(&addr).serve(app.into_make_service());
    let actual_addr = server.local_addr();

    // Spawn server task
    tokio::spawn(async move {
        server.await.expect("Server task failed");
    });

    actual_addr
}
