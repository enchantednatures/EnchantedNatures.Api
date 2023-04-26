use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Photo {
    pub id: i32,
    pub description: String,
    pub date_taken: String,
    pub cdn_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: i32,
    pub name: String,
}
