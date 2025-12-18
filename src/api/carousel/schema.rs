use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CarouselItem {
    pub id: i32,
    pub image_url: String,
    pub title: String,
    pub subtitle: String,
    pub link: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCarouselBody {
    pub image_url: String,
    pub title: String,
    pub subtitle: String,
    pub link: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCarouselBody {
    pub image_url: Option<String>,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub link: Option<String>,
}
