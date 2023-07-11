use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::models::Photo;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PhotoCreateRequest {
    pub name: String,
    pub description: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PhotoCreatedResponse {
    pub photo: Photo,
}
