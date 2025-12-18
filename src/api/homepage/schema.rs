use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HomepageContentRow {
    pub section_name: String,
    pub content: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpsertHomepageBody {
    pub content: String,
}
