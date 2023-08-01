use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{response, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdatePhotoCategoryRequest {
    pub photo_id: i32,
    // pub display_order: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum PatchCategoryRequestBody {
    AddPhotoToCategory(UpdatePhotoCategoryRequest),
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PhotoAddedToCategory;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PhotoRemovedFromCategory;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum UpdatePhotoCategoryResponse {
    PhotoAddedToCategory,
    PhotoRemovedFromCategory,
}

#[utoipa::path(
    patch,
    path = "/api/v0/categories/{id}",
    params(
        ("id"=i32, Path, description = "Update category")
    ),
    responses(
        (status = StatusCode::OK, description = "PhotoCategory successfully updated", body = UpdatePhotoCategoryResponse),
        (status = StatusCode::NOT_FOUND, description = "PhotoCategory not found")
    )
)]
pub async fn patch_category(
    State(db_pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<PatchCategoryRequestBody>,
) -> response::Result<impl IntoResponse, (StatusCode, String)> {
    match payload {
        PatchCategoryRequestBody::AddPhotoToCategory(p) => {
            sqlx::query!(
                r#"
INSERT INTO public.photo_categories (photo_id, category_id, display_order)
VALUES (
        $1, $2, ((SELECT coalesce(max(display_order), 0) as max_display_order
          FROM photo_categories
          WHERE category_id = 1
          GROUP BY category_id) + 1
            ))
                "#,
                p.photo_id,
                id
            )
            .execute(&db_pool)
            .await
            .unwrap();
        }
    };
    Ok((StatusCode::OK, Json(json!({"status": "not implemented"}))))
}
