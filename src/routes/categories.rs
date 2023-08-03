use crate::database::PhotoRepo;
use crate::Database;
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{response, Extension, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{query_as, PgPool};
use utoipa::{IntoResponses, ToSchema};

use crate::models::{Category, CategoryDisplayModel, CategoryViewModel, Photo, PhotoViewModel};

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub struct AddPhotoToCategoryRequest {
    pub photo_id: i32,
    pub display_order: Option<i32>,
}

#[utoipa::path(
    post,
    path = "/api/v0/categories/{id}",
    request_body = AddPhotoToCategoryRequest,
    params(
        ("id"= i32, Path, description = "Id of category to get photos for")
    ),
    responses(
        (status = StatusCode::ACCEPTED, description = "Check health"),
    )
)]
pub async fn add_photo_to_category(
    State(db_pool): State<PgPool>,
    Path(category_id): Path<i32>,
    Json(request): Json<AddPhotoToCategoryRequest>,
) -> response::Result<impl IntoResponse, (StatusCode, String)> {
    println!("{}", &category_id);
    println!("{:?}", &request);
    let mut transaction = db_pool.begin().await.unwrap();

    // Replace "your_table" and "your_field" with your actual table and field names
    // Also replace "your_condition" with your actual condition
    let row = sqlx::query!(
        r#"SELECT MAX(display_order) as max_display_order FROM photo_categories WHERE category_id = $1"#,
        &category_id
    )
        .fetch_one(&mut *transaction)
        .await
        .unwrap();

    let max_value = row.max_display_order.unwrap_or(0); // Use 0 if there are no rows

    if let Some(display) = request.display_order {
        if display <= max_value {
            sqlx::query!(
                r#"
                UPDATE photo_categories
                SET display_order = display_order + 1
                WHERE category_id = $1
                AND display_order > $2
           "#,
                &category_id,
                &display
            )
            .execute(&mut *transaction)
            .await
            .unwrap();
        }
    }

    // Now insert a new row with `max_value + 1` as the value for `your_field`
    // Replace "your_values" with the values for the other fields in your table
    sqlx::query!(
        "INSERT INTO photo_categories (category_id, photo_id, display_order) VALUES ($1, $2, $3)",
        &category_id,
        &request.photo_id,
        &max_value + 1
    )
    .execute(&mut *transaction)
    .await
    .unwrap();

    transaction.commit().await.unwrap();
    Ok(StatusCode::NOT_IMPLEMENTED)
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
