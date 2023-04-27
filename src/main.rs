use actix_web::middleware::Logger;
use actix_web::{main, App, HttpServer};
use dotenv::dotenv;
use paperclip::actix::{api_v2_definition, api_v2_operation, web::Json, OpenApiExt, Resource};
use sqlx::postgres::PgPoolOptions;
use std::env;

mod models;
mod routes;
mod view_models;

mod db;

#[main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create connection pool");

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_owned());
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_owned())
        .parse::<u16>()
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(pool.clone())
            .wrap_api()
            .with_json_spec_at("/api/swagger.json")
            .configure(routes::configure)
            .build()
    })
    .bind((host, port))?
    .run()
    .await
}
