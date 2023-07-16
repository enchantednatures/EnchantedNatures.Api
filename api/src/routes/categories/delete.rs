use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct DeleteCategoryResponse {
    status: String,
}

#[utoipa::path(
    delete,
    path = "/api/v0/categories/{id}",
    params(
        ("id" = i32, Path, description = "Id of category to delete" ),
    ),
    responses(
        (status = StatusCode::NO_CONTENT, description = "Category Deleted"),
        (status = StatusCode::NOT_FOUND, description = "Category Not Found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Server Error"),
    )
)]
async fn delete(
    State(db_pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: verify a row was deleted
    sqlx::query!(
        r#"
    DELETE FROM categories
    WHERE id = $1
    "#,
        id
    )
    .execute(&db_pool)
    .await
    .unwrap();

    Ok(StatusCode::NO_CONTENT)
}
