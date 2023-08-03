use crate::models::{Category, Photo};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct PhotoViewModel {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub url: String,
}

impl From<Photo> for PhotoViewModel {
    fn from(value: Photo) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
            url: value.url,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CategoryViewModel {
    pub id: i32,
    pub name: String,
    pub description: String,
}

impl From<Category> for CategoryViewModel {
    fn from(value: Category) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct PhotoDisplayModel {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub url: String,
    pub categories: Vec<CategoryViewModel>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CategoryDisplayModel {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub photos: Vec<PhotoViewModel>,
}
