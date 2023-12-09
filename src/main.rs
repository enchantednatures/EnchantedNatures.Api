#![warn(dead_code)]

use enchantednatures::app::create_router;

use enchantednatures::database::PhotoRepository;
use enchantednatures::domain::AppState;

use shuttle_runtime::CustomError;

use sqlx::PgPool;

use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
// use tracing_subscriber::layer::SubscriberExt;
// use tracing_subscriber::EnvFilter;
// use tracing_subscriber::Registry;
use utoipa_swagger_ui::{Config, SwaggerUi};

// fn setup_logging() {
//     let formatting_layer = BunyanFormattingLayer::new("enchanted_natures".into(), std::io::stdout);
//     let subscriber = Registry::default()
//         .with(JsonStorageLayer)
//         .with(EnvFilter::new("info"))
//         .with(formatting_layer);

//     tracing::subscriber::set_global_default(subscriber).unwrap();
// }

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    // setup_logging();

    sqlx::migrate!()
        .run(&pool)
        .await
        .map_err(CustomError::new)?;

    let photo_repo = PhotoRepository::new(pool);
    let app_state = AppState::new(photo_repo);
    let swagger_config = Config::from("/enchanted-natures.openapi.spec.yaml");
    let swagger_ui = SwaggerUi::new("/swagger-ui").config(swagger_config);
    let app = create_router(swagger_ui, app_state);
    Ok(app.into())
}
