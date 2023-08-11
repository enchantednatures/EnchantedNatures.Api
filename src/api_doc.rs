use utoipa::OpenApi;

use crate::routes::categories;
use crate::routes::health;
use crate::routes::photos;
use crate::routes::upload;

use crate::models;

#[derive(OpenApi)]
#[openapi(
    paths(
        health::health_check,
        categories::categories_by_id,
        categories::get_categories,
        categories::post_category,
        categories::add_photo_to_category,
        categories::delete,
        photos::post_photo,
        photos::get_photo,
        photos::get_photos,
        photos::delete_photo,
        upload::save_request_body
    ),
    components(
        schemas(
            models::CategoryViewModel,
            models::CategoryDisplayModel,
            models::PhotoViewModel,
            models::PhotoDisplayModel,
            models::Photo,
            models::Category,
            photos::PhotoCreateRequest,
            categories::CategoryError,
            categories::AddPhotoToCategoryRequest,
            categories::UpdatePhotoCategoryResponse,
            categories::CategoryGetByIdRequest,
            categories::CreateCategoryRequest,
            categories::CategoryGetByIdResponse,
            health::HealthStatus,
            health::HealthStatusEnum,
            upload::UploadedPhotoViewModel
        ),
    ),
    tags(
        (name = "Health Checks", description = "Information about the health of the API")
    )
)]
pub(crate) struct ApiDoc;
