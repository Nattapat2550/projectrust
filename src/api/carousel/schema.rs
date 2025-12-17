use serde::{Deserialize, Serialize};
use validator::Validate;
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct CarouselItem {
    pub id: i32,
    pub item_index: i32,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub image_dataurl: String, // ตรงกับ DB
}

#[derive(Deserialize, Validate)]
pub struct CreateCarouselPayload {
    pub item_index: Option<i32>,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    
    #[validate(length(min = 1))]
    pub image_dataurl: String, // รับเป็น Base64 Data URL
}