// use crate::auth::User;
use crate::database::PhotoRepo;

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
    use axum::routing::post;
    Router::new()
        .route("/categories", get(get_categories)//.post(post_category)
        )
        .route(
            "/categories/:id",
            get(categories_by_id)//.delete(delete_category),
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

#[tracing::instrument(name = "add photo to category", skip(app))]
pub async fn add_photo_to_category(
    State(app): State<AppState>,
    Path(category_id): Path<i32>,
    // user: User,
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

#[tracing::instrument(name = "add category", skip(app))]
pub async fn post_category(
    State(app): State<AppState>,
    // user: User,
    Json(payload): Json<CreateCategoryRequest>,
) -> response::Result<impl IntoResponse, (StatusCode, String)> {
    let category: CategoryViewModel = app.repo.add_category(payload.name).await.unwrap().into();
    let redirect_url = format!("/{}", category.id);
    let mut response_headers: HeaderMap = HeaderMap::new();
    response_headers.insert(
        header::LOCATION,
        HeaderValue::from_str(redirect_url.as_str()).unwrap(),
    );

    Ok((StatusCode::CREATED, response_headers, Json(category)))
}

#[tracing::instrument(name = "Delete Category", skip(app))]
pub async fn delete_category(
    State(app): State<AppState>,
    Path(id): Path<i32>,
) -> response::Result<impl IntoResponse, (StatusCode, String)> {
    // TODO: verify a row was deleted
    sqlx::query!(
        r#" DELETE FROM categories
            WHERE id = $1 "#,
        id
    )
    .execute(&*app.repo.db_pool)
    .await
    .unwrap();

    Ok(StatusCode::NO_CONTENT)
}
