use std::env;

use sqlx::{Connection, PgConnection, PgPool};

pub async fn create_connection_pool() -> PgPool {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPool::connect(&db_url)
        .await
        .expect("Failed to create connection pool")
}
