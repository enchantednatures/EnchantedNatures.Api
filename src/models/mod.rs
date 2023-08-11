use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use utoipa::ToSchema;

pub use view_models::*;

mod view_models;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Photo {
    pub id: i32,
    pub title: String,
    pub location_taken: String,
    pub filename: String,
    pub date_taken: NaiveDate,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Category {
    pub fn new(
        id: i32,
        name: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            name,
            created_at,
            updated_at,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct PhotoCategory {
    pub id: i32,
    pub display_order: i32,
    pub photo_id: i32,
    pub category_id: i32,
}
