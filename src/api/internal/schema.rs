use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FindUserBody {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLite {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub role: String,
    pub provider: String,
    pub is_verified: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetClientActiveBody {
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientRow {
    pub id: i32,
    pub name: String,
    pub api_key: String,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HomepageHero {
    pub title: String,
    pub subtitle: String,
    pub cta_text: String,
    pub cta_link: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HomepageHeroBody {
    pub title: String,
    pub subtitle: String,
    pub cta_text: String,
    pub cta_link: String,
}

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
