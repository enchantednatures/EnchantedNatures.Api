use crate::database::PhotoRepo;
use crate::App;

use crate::Database;
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{response, Extension, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;

use utoipa::{IntoResponses, ToSchema};

use crate::models::{Category, CategoryViewModel, Photo};

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub struct AddPhotoToCategoryRequest {
    pub photo_id: i32,
    pub display_order: Option<i32>,
}

#[utoipa::path(
    post,
    path = "/api/v0/categories/{id}/photos",
    request_body = AddPhotoToCategoryRequest,
    params(
        ("id"= i32, Path, description = "Category to add photo to")
    ),
    responses(
        (status = StatusCode::ACCEPTED, description = "Photo added to category"),
    )
)]
pub async fn add_photo_to_category(
    State(app): State<App>,
    Path(category_id): Path<i32>,
    Json(request): Json<AddPhotoToCategoryRequest>,
) -> response::Result<impl IntoResponse, (StatusCode, String)> {
    match app
        .repo
        .add_photo_to_category(request.photo_id, category_id, request.display_order)
        .await
    {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(e) => {
            tracing::error!("Failed to add photo to category: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to add photo to category: {}", e),
            ))
        }
    }
}

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
    path = "/api/v0/categories",
    responses(
        (status = StatusCode::OK, description = "Get all categories", body = [Category]),
    )
)]
pub async fn get_categories(
    State(app): State<App>,
) -> response::Result<impl IntoResponse, (StatusCode, String)> {
    match app.repo.get_categories().await {
        Ok(resp) => Ok((StatusCode::OK, Json(resp))),
        Err(e) => {
            tracing::error!("Failed to get categories: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to get categories: {}", e),
            ))
        }
    }
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
    State(app): State<App>,
    Path(id): Path<i32>,
) -> response::Result<impl IntoResponse, (StatusCode, String)> {
    match app.repo.get_category(id).await {
        Ok(resp) => Ok((StatusCode::OK, Json(resp))),
        Err(e) => {
            tracing::error!("Failed to get category: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to get category: {}", e),
            ))
        }
    }
}

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
    path = "/api/v0/categories",
    request_body = CreateCategoryRequest,
    responses(
        (status = 201, description = "Category created successfully", body = CategoryViewModel),
        (status = 409, description = "Category already exists", body = CategoryError),
    )
)]
pub async fn put_category(
    Extension(repo): Extension<Database>,
    Json(payload): Json<CreateCategoryRequest>,
) -> response::Result<impl IntoResponse, (StatusCode, String)> {
    let category: CategoryViewModel = repo
        .add_category(payload.name, payload.description)
        .await
        .unwrap()
        .into();

    Ok((StatusCode::CREATED, Json(category)))
}

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
    post,
    path = "/api/v0/categories/{id}",
    params(
        ("id"=i32, Path, description = "Update category")
    ),
    responses(
        (status = StatusCode::OK, description = "PhotoCategory successfully updated", body = UpdatePhotoCategoryResponse),
        (status = StatusCode::NOT_FOUND, description = "PhotoCategory not found")
    )
)]
pub async fn post_category(
    State(app): State<App>,
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
            .execute(&*app.repo.db_pool)
            .await
            .unwrap();
        }
    };
    Ok((StatusCode::OK, Json(json!({"status": "not implemented"}))))
}

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
    path = "/api/v0/categories/{id}/photos",
    params(
        ("id" = i32, Path, description = "Id of category to delete" ),
    ),
    responses(
        DeleteCategoryResponse,
    )
)]
async fn delete(
    State(app): State<App>,
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
    .execute(&*app.repo.db_pool)
    .await
    .unwrap();

    Ok(StatusCode::NO_CONTENT)
}
