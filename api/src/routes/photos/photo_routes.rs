use axum::extract::State;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Json, response};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;
use crate::routes::HealthStatus;
use crate::routes::PatchCategoryRequestBody::AddPhotoToCategory;

use crate::models::{Category, Photo};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdatePhotoCategoryRequest {
    pub photo_id: i32,
    // pub display_order: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PhotoGetAllResponse;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum PatchCategoryRequestBody {
    AddPhotoToCategory(UpdatePhotoCategoryRequest),
}

#[utoipa::path(get, path = "/api/v0/categories/",
responses(
(status = StatusCode::OK, description = "Get all categories", body = [Category]),))]
pub async fn get_categories(
    State(db_pool): State<PgPool>,
) -> response::Result<impl IntoResponse, (StatusCode, String)> {
    // let mut response: Vec<Category> = vec![];
    let response = sqlx::query_as!(
        Category,
        r#"
            SELECT id as "id!",
               name as "name!",
               description as "description!",
               created_at as "created_at!",
               updated_at as "updated_at!"
            FROM public.categories "#
    )
        .fetch_all(&db_pool)
        .await
        .unwrap();
    Ok((StatusCode::OK, Json(response)))
}

#[utoipa::path(
patch,
path = "/categories/{id}/",
responses(
(status = StatusCode::OK, description = "Check health", body = HealthStatus),)
)
]
pub async fn patch_category(
    State(db_pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<PatchCategoryRequestBody>,
) -> response::Result<impl IntoResponse, (StatusCode, String)> {
    match payload {
        AddPhotoToCategory(p) => {
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
    Ok((StatusCode::OK, Json(HealthStatus::new())))
}

#[utoipa::path(get, path = "/categories/{id}/", responses((status = StatusCode::OK, description = "Check health", body = HealthStatus),))]
pub async fn get_photos_in_category(
    State(db_pool): State<PgPool>,
    Path(id): Path<i32>,
) -> response::Result<impl IntoResponse, (StatusCode, String)> {
    Ok((StatusCode::OK, Json(HealthStatus::new())))
}
