use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{response, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PhotoGetAllResponse;

#[utoipa::path(
    delete,
    path = "/api/v0/photos/:id",
    responses(
        (status = StatusCode::NO_CONTENT, description = "Delete photo with Id"),
    )
)]
pub async fn delete_photo(
    State(db_pool): State<PgPool>,
    Path(id): Path<i32>,
) -> response::Result<impl IntoResponse, (StatusCode, String)> {
    // let mut response: Vec<Photo> = vec![];
    sqlx::query!(
        r#"
            DELETE  
            FROM public.photos 
            WHERE id = $1 
        "#,
        &id
    )
    .execute(&db_pool)
    .await
    .unwrap();
    Ok((StatusCode::NO_CONTENT, Json(json!({ "deleted": &id }))))
}
