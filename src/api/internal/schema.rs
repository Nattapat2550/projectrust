use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct FindUserBody {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserEmailBody {
    pub email: String,
}

// ✅ เพิ่ม Struct นี้
#[derive(Debug, Deserialize)]
pub struct SetOAuthUserBody {
    pub email: String,
    pub provider: String,
    pub oauth_id: String,
    pub picture_url: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserLite {
    pub id: i32,
    pub email: String,
    pub username: Option<String>,
    pub role: String,
    pub provider: Option<String>,
    pub is_verified: bool,
}

#[derive(Debug, Serialize)]
pub struct ClientRow {
    pub id: i32,
    pub name: String,
    pub api_key: String,
    pub is_active: bool,
}

#[derive(Debug, Serialize)]
pub struct HomepageHero {
    pub title: String,
    pub subtitle: Option<String>,
    pub cta_text: Option<String>,
    pub cta_link: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct HomepageHeroBody {
    pub title: String,
    pub subtitle: String,
    pub cta_text: String,
    pub cta_link: String,
}

#[derive(Debug, Serialize)]
pub struct CarouselItem {
    pub id: i32,
    pub image_url: String,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub link: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCarouselBody {
    pub image_url: String,
    pub title: String,
    pub subtitle: String,
    pub link: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCarouselBody {
    pub image_url: Option<String>,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub link: Option<String>,
}