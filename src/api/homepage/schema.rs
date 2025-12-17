use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct HomepageContent {
    pub section_name: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct UpdateContentPayload {
    pub content: String,
}