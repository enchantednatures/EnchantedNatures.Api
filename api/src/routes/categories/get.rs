use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{response, Json};
use serde::{Deserialize, Serialize};
use sqlx::{query_as, PgPool};
use utoipa::ToSchema;

use crate::models::{Category, CategoryDisplayModel, Photo, PhotoViewModel};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CategoryGetByIdRequest {
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CategoryGetByIdResponse {
    pub category: Category,
    pub photos: Vec<Photo>,
}

#[utoipa::path(
    get,
    path = "/api/v0/categories/",
    responses(
        (status = StatusCode::OK, description = "Get all categories", body = [Category]),
    )
)]
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
    get,
    path = "/api/v0/categories/{id}",
    params(
        ("id"= i32, Path, description = "Id of category to get photos for")  
    ),
    responses(
        (status = StatusCode::OK, description = "Check health", body = CategoryDisplayModel),
    )
)]
pub async fn categories_by_id(
    State(db_pool): State<PgPool>,
    Path(id): Path<i32>,
) -> response::Result<impl IntoResponse, (StatusCode, String)> {
    let category = query_as!(
        Category,
        r#"
           SELECT id as "id!",
               name as "name!",
               description as "description!",
               created_at as "created_at!",
               updated_at as "updated_at!"
            FROM public.categories
            WHERE id = $1
            "#,
        &id
    )
    .fetch_one(&db_pool)
    .await
    .unwrap();

    let photos: Vec<PhotoViewModel> = query_as!(
        PhotoViewModel,
        r#"
SELECT id as "id!",
       name as "name!",
       description as "description!",
       url as "url!"
FROM photos
WHERE photos.id in (SELECT DISTINCT photo_id
                    FROM photo_categories
                    WHERE category_id = $1);
        "#,
        &category.id
    )
    .fetch_all(&db_pool)
    .await
    .unwrap();

    let display = CategoryDisplayModel {
        id: category.id,
        name: category.name,
        description: category.description,
        photos,
    };

    Ok((StatusCode::OK, Json(display)))
}
