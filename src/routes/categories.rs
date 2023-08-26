use crate::database::PhotoRepo;
use crate::models::Category;
use crate::models::CategoryDisplayModel;
use crate::models::CategoryViewModel;
use crate::models::Photo;

use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{response, Json};
use serde::{Deserialize, Serialize};

use crate::domain::AppState;
use tracing::info;

#[derive(Deserialize, Serialize, Debug)]
pub struct AddPhotoToCategoryRequest {
    pub photo_id: i32,
    pub display_order: Option<i32>,
}

#[tracing::instrument(name = "add photo to category", skip(app))]
pub async fn add_photo_to_category(
    State(app): State<AppState>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryGetByIdRequest {
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryGetByIdResponse {
    pub category: Category,
    pub photos: Vec<Photo>,
}

#[tracing::instrument(name = "Get Categories", skip(app))]
pub async fn get_categories(
    State(app): State<AppState>,
) -> response::Result<impl IntoResponse, (StatusCode, String)> {
    match app.repo.get_categories().await {
        Ok(resp) => {
            info!("got {} categories", resp.len());
            info!("{:?}", resp);
            Ok((StatusCode::OK, Json(resp)))
        }
        Err(e) => {
            tracing::error!("Failed to get categories: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to get categories: {}", e),
            ))
        }
    }
}

#[tracing::instrument(name = "Get Category", skip(app))]
pub async fn categories_by_id(
    State(app): State<AppState>,
    Path(id): Path<i32>,
) -> response::Result<impl IntoResponse, (StatusCode, String)> {
    match app.repo.get_category(id).await {
        Ok(resp) => {
            info!("Category retrieved successfully");
            Ok((StatusCode::OK, Json(CategoryDisplayModel::from(resp))))
        }
        Err(e) => {
            tracing::error!("Failed to get category: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to get category: {}", e),
            ))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCategoryRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CategoryError {
    CategoryAlreadyExists,
}

#[tracing::instrument(name = "add category", skip(app))]
pub async fn post_category(
    State(app): State<AppState>,
    Json(payload): Json<CreateCategoryRequest>,
) -> response::Result<impl IntoResponse, (StatusCode, String)> {
    let category: CategoryViewModel = app.repo.add_category(payload.name).await.unwrap().into();

    Ok((StatusCode::CREATED, Json(category)))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhotoAddedToCategory;

#[derive(Debug, Serialize, Deserialize)]
pub struct PhotoRemovedFromCategory;

#[derive(Debug, Serialize, Deserialize)]
pub enum UpdatePhotoCategoryResponse {
    PhotoAddedToCategory,
    PhotoRemovedFromCategory,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeleteCategoryResponses {
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct BadRequest {
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
enum DeleteCategoryResponse {
    Success,

    NotFound,

    BadRequest(BadRequest),
}

pub async fn delete_category(
    State(app): State<AppState>,
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
