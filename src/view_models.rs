use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryViewModel {
    pub id: Option<Uuid>,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhotoViewModel {
    pub id: Option<Uuid>,
    pub description: String,
    pub date_taken: chrono::NaiveDateTime,
    pub cdn_path: String,
}
impl PhotoViewModel {
    pub fn new(
        id: Option<Uuid>,
        description: String,
        date_taken: chrono::NaiveDateTime,
        cdn_path: String,
    ) -> Self {
        Self {
            id,
            description,
            date_taken,
            cdn_path,
        }
    }
}

impl From<Photo> for PhotoViewModel {
    fn from(photo: Photo) -> Self {
        Self::new(
            Some(photo.id),
            photo.description,
            photo.date_taken,
            photo.cdn_path,
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryPhotoViewModel {
    pub category_id: Uuid,
    pub photo_id: Uuid,
    pub order_in_category: i32,
}
