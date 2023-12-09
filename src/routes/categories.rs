use crate::database::PhotoRepository;
use crate::models::CategoryDisplayModel;
use crate::models::CategoryViewModel;

use axum::extract::Path;
use axum::extract::State;
use axum::http::header;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;
use axum::{response, Json};
use hyper::http::HeaderValue;
use hyper::HeaderMap;
use serde::{Deserialize, Serialize};

use crate::domain::AppState;
use tracing::info;

pub fn categories_router() -> Router<AppState> {
    use axum::routing::get;

    Router::new()
        .route(
            "/categories",
            get(get_categories), //.post(post_category)
        )
        .route(
            "/categories/:id",
            get(categories_by_id), //.delete(delete_category),
        )
    //.route("/categories/:id/photos", post(add_photo_to_category))
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AddPhotoToCategoryRequest {
    pub photo_id: i32,
    pub display_order: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCategoryRequest {
    pub name: String,
}

#[tracing::instrument(name = "add photo to category", skip(photo_repo))]
pub async fn add_photo_to_category(
    State(photo_repo): State<PhotoRepository>,
    Path(category_id): Path<i32>,
    // user: User,
    Json(request): Json<AddPhotoToCategoryRequest>,
) -> response::Result<impl IntoResponse, (StatusCode, String)> {
    match photo_repo
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

#[tracing::instrument(name = "Get Categories", skip(photo_repo))]
pub async fn get_categories(
    State(photo_repo): State<PhotoRepository>,
) -> response::Result<impl IntoResponse, (StatusCode, String)> {
    match photo_repo.get_categories().await {
        Ok(resp) => {
            info!("got {} categories", resp.len());
            // info!("{:?}", resp);
            let view_model: Vec<CategoryViewModel> = resp.into_iter().map(|x| x.into()).collect();
            Ok((StatusCode::OK, Json(view_model)))
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

#[tracing::instrument(name = "Get Category", skip(photo_repository))]
pub async fn categories_by_id(
    State(photo_repository): State<PhotoRepository>,
    Path(id): Path<i32>,
) -> response::Result<impl IntoResponse, (StatusCode, String)> {
    match photo_repository.get_category(id).await {
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

#[tracing::instrument(name = "add category", skip(photo_repository))]
pub async fn post_category(
    State(photo_repository): State<PhotoRepository>,
    // user: User,
    Json(payload): Json<CreateCategoryRequest>,
) -> response::Result<impl IntoResponse, (StatusCode, String)> {
    let category: CategoryViewModel = photo_repository
        .add_category(payload.name)
        .await
        .unwrap()
        .into();
    let redirect_url = format!("/{}", category.id);
    let mut response_headers: HeaderMap = HeaderMap::new();
    response_headers.insert(
        header::LOCATION,
        HeaderValue::from_str(redirect_url.as_str()).unwrap(),
    );

    Ok((StatusCode::CREATED, response_headers, Json(category)))
}

#[tracing::instrument(name = "Delete Category", skip(photo_repository))]
pub async fn delete_category(
    State(photo_repository): State<PhotoRepository>,
    // _user: User,
    Path(id): Path<i32>,
) -> response::Result<impl IntoResponse, (StatusCode, String)> {
    // TODO: verify a row was deleted
    sqlx::query!(
        r#" DELETE FROM categories
            WHERE id = $1 "#,
        id
    )
    .execute(&*photo_repository.db_pool)
    .await
    .unwrap();

    Ok(StatusCode::NO_CONTENT)
}
