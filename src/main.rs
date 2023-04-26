mod db;
mod routes;

use actix_web::{routes, App, HttpServer};
use dotenv::dotenv;
use actix_web::{App, HttpServer};
use paperclip::actix::{
    api_v2_schema, // Macro to serve the API spec
    web::{self, Data}, // Use Paperclip's web module
    OpenApiExt, // Extension trait
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // ... other configurations ...

    HttpServer::new(move || {
        App::new()
            .wrap_api()
            .with_json_spec_at("/api/spec") // Serve the API spec at /api/spec
            .app_data(Data::new(pool.clone())) // Use Paperclip's Data instead of Actix-web's Data
            .service(get_categories)
            .service(put_category)
            // ... other services ...
            .build()
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
