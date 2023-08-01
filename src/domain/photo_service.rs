use anyhow::Result;
use async_trait::async_trait;

use crate::database::{PhotoRepo, PhotoRepository};

// macro?
pub struct CreatePhotoRequest {
    name: String,
    description: String,
    url: String,
}
struct CreatePhotoResponse;
struct GetPhotosRequest;
struct GetPhotosResponse;
struct GetPhotoRequest;
struct GetPhotoResponse;
struct UpdatePhotoRequest;
struct UpdatePhotoResponse;
struct DeletePhotoRequest;
struct DeletePhotoResponse;

struct CreateCategoryRequest;
struct CreateCategoryResponse;
struct GetCategoriesRequest;
struct GetCategoriesResponse;
struct GetCategoryRequest;
struct GetCategoryResponse;
struct UpdateCategoryRequest;
struct UpdateCategoryResponse;
struct DeleteCategoryRequest;
struct DeleteCategoryResponse;

#[async_trait]
trait PhotoService {
    async fn create_photo(&self, photo: CreatePhotoRequest) -> Result<CreatePhotoResponse>;
    async fn delete_photo(&self, photo_id: i32) -> DeletePhotoResponse;
}

pub struct PhotoValidator;
impl PhotoValidator {
    pub fn validate_photo(&self, photo: &CreatePhotoRequest) -> bool {
        photo.name.len() > 0 && photo.description.len() > 0 && photo.url.len() > 0
    }
}

pub struct Photos {
    repo: PhotoRepository,
    validator: PhotoValidator,
}

impl Photos {
    fn new(repo: PhotoRepository, validator: PhotoValidator) -> Photos {
        Self { repo, validator }
    }
}

#[async_trait]
impl PhotoService for Photos {
    async fn create_photo(&self, photo: CreatePhotoRequest) -> Result<CreatePhotoResponse> {
        let is_valid = self.validator.validate_photo(&photo);
        if !is_valid {}
        let added_photo = self
            .repo
            .add_photo(photo.name, photo.description, photo.url)
            .await
            .unwrap();
        // TODO: implement
        Ok(CreatePhotoResponse {})
    }

    async fn delete_photo(&self, photo_id: i32) -> DeletePhotoResponse {
        todo!()
    }
}
