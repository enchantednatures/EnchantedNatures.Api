use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct PhotoViewModel {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CategoryViewModel {
    pub id: i32,
    pub name: String,
    pub description: String,
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
