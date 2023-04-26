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

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryPhotoViewModel {
    pub category_id: Uuid,
    pub photo_id: Uuid,
    pub order_in_category: i32,
}
