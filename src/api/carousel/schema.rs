use serde::{Deserialize, Serialize};
use validator::Validate;
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct CarouselItem {
    pub id: i32,
    pub image_url: String,
    pub link: Option<String>,
    pub is_active: bool,
}

#[derive(Deserialize, Validate)]
pub struct CreateCarouselPayload {
    #[validate(url(message = "Invalid URL format"))]
    pub image_url: String,
    
    pub link: Option<String>,
}