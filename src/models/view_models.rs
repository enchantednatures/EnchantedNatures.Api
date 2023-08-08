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

impl PhotoViewModel {
    pub fn new(id: i32, name: String, description: String, url: String) -> Self {
        Self {
            id,
            name,
            description,
            url,
        }
    }
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

pub type CategoryPhotos = (Category, Vec<Photo>);

impl From<CategoryPhotos> for CategoryDisplayModel {
    fn from(value: CategoryPhotos) -> Self {
        CategoryDisplayModel {
            id: value.0.id,
            name: value.0.name,
            description: value.0.description,
            photos: value
                .1
                .into_iter()
                .map(|x| PhotoViewModel::new(x.id, x.name, x.description, x.url))
                .collect(),
        }
    }
}
