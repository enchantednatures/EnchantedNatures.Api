use serde::{Deserialize, Serialize};
use axum::extract::State;
use sqlx::PgPool;
use axum::{Json, response};
use axum::response::IntoResponse;
use axum::http::StatusCode;
use utoipa::ToSchema;
use crate::models::Category;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateCategoryRequest {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum CategoryError {
    CategoryAlreadyExists,
}

#[utoipa::path(
put,
path = "/api/v0/categories/",
request_body = CreateCategoryRequest,
responses(
(status = 201, description = "Category created successfully", body = Category),
(status = 409, description = "Category already exists", body = CategoryError),
)
)]
pub async fn put_category(
    State(db_pool): State<PgPool>,
    Json(payload): Json<CreateCategoryRequest>,
) -> response::Result<impl IntoResponse, (StatusCode, String)> {
    let category = sqlx::query_as!(
        Category,
        r#"
        INSERT INTO public.categories (name, description)
        VALUES ($1, $2) RETURNING id as "id!",
               name as "name!",
               description as "description!",
               created_at as "created_at!",
               updated_at as "updated_at!"
        "#,
        payload.name,
        payload.description
    )
        .fetch_one(&db_pool)
        .await
        .unwrap();

    Ok((StatusCode::CREATED, Json(category)))
}