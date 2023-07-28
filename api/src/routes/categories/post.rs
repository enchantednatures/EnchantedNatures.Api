use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{response, Json};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa;
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AddPhotoToCategoryRequest {
    pub photo_id: i32,
    pub display_order: Option<i32>,
}

#[derive(sqlx::Type)]
#[sqlx(transparent)]
struct Id(i32);

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
