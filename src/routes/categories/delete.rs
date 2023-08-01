use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::{IntoResponses, ToSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct DeleteCategoryResponses {
    status: String,
}
enum DeleteCategoryStatus {
    Success,
    NotFound,
    ServerError,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct BadRequest {
    message: String,
}

#[derive(Debug, Serialize, Deserialize, IntoResponses)]
#[response(description = "Delete a category", content_type = "application/json")]
enum DeleteCategoryResponse {
    #[response(status = 200, description = "Category Deleted")]
    Success,

    #[response(status = StatusCode::NOT_FOUND, description = "Category Not Found")]
    NotFound,

    #[response(status = StatusCode::INTERNAL_SERVER_ERROR, description = "Server Error")]
    BadRequest(BadRequest),
}

#[utoipa::path(
    delete,
    path = "/api/v0/categories/{id}",
    params(
        ("id" = i32, Path, description = "Id of category to delete" ),
    ),
    responses(
        DeleteCategoryResponse,
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
