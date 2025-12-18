use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Same shape as pure-api1 (Postgres columns are snake_case)
#[derive(Debug, Serialize, Deserialize)]
pub struct CarouselItem {
    pub id: i32,
    pub item_index: i32,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub image_dataurl: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCarouselBody {
    #[serde(alias = "itemIndex")]
    pub item_index: Option<i32>,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    #[serde(alias = "imageDataUrl", alias = "image_data_url", alias = "imageDataurl")]
    pub image_dataurl: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCarouselBody {
    #[serde(alias = "itemIndex")]
    pub item_index: Option<i32>,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    #[serde(alias = "imageDataUrl", alias = "image_data_url", alias = "imageDataurl")]
    pub image_dataurl: Option<String>,
}
