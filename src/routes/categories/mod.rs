use axum::routing::get;
use axum::Router;
use sqlx::PgPool;

pub use get::*;
pub use patch::*;
pub use post::*;
pub use put::*;

pub mod delete;
pub mod get;
pub mod patch;
pub mod post;
pub mod put;

pub fn router(db: PgPool) -> Router {
    Router::new()
        .route("/api/v0/categories", get(get_categories).put(put_category))
        .route(
            "/api/v0/categories/:id",
            get(categories_by_id).patch(patch_category),
        )
        .with_state(db)
}
