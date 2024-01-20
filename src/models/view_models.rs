use crate::models::{Category, Photo};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PhotoViewModel {
    pub id: i32,
    pub title: String,
    pub filename: String,
    pub location_taken: String,
    pub date_taken: NaiveDate,
}

impl PhotoViewModel {
    pub fn new(
        id: i32,
        title: String,
        filename: String,
        location_taken: String,
        date_taken: NaiveDate,
    ) -> Self {
        Self {
            id,
            title,
            filename,
            location_taken,
            date_taken,
        }
    }
}

impl From<Photo> for PhotoViewModel {
    fn from(value: Photo) -> Self {
        Self {
            id: value.id,
            title: value.title,
            filename: value.filename,
            location_taken: value.location_taken,
            date_taken: value.date_taken,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CategoryViewModel {
    pub id: i32,
    pub name: String,
}

impl From<Category> for CategoryViewModel {
    fn from(value: Category) -> Self {
        Self {
            id: value.id,
            name: value.name,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PhotoDisplayModel {
    pub id: i32,
    pub title: String,
    pub name: String,
    pub filename: String,
    pub url: String,
    pub categories: Vec<CategoryViewModel>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CategoryDisplayModel {
    pub id: i32,
    pub name: String,
    pub photos: Vec<PhotoViewModel>,
}

pub type CategoryPhotos = (Category, Vec<Photo>);

impl From<CategoryPhotos> for CategoryDisplayModel {
    fn from(value: CategoryPhotos) -> Self {
        CategoryDisplayModel {
            id: value.0.id,
            name: value.0.name,
            photos: value
                .1
                .into_iter()
                .map(|x| {
                    PhotoViewModel::new(x.id, x.title, x.filename, x.location_taken, x.date_taken)
                })
                .collect(),
        }
    }
}
