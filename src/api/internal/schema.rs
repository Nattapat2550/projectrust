use serde::{Deserialize, Serialize};

// --- User Schemas ---

#[derive(Debug, Deserialize)]
pub struct FindUserBody {
    pub email: Option<String>,
    pub id: Option<i32>,
    pub provider: Option<String>,
    pub oauth_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserLite {
    pub id: i32,
    pub email: String,
    pub username: Option<String>,
    pub role: String,
    pub provider: Option<String>,
    pub is_verified: bool,
    pub profile_picture_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserEmailBody {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct SetOAuthUserBody {
    pub email: String,
    pub provider: String,
    pub oauth_id: String,
    pub picture_url: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SetUsernamePasswordBody {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserBody {
    pub id: i32,
    pub username: Option<String>,
    pub profile_picture_url: Option<String>,
}

// --- Verification & Reset ---

#[derive(Debug, Deserialize)]
pub struct StoreVerificationCodeBody {
    pub user_id: i32,
    pub code: String,
    pub expires_at: String, // ISO string
}

#[derive(Debug, Deserialize)]
pub struct VerifyCodeBody {
    pub email: String,
    pub code: String,
}

#[derive(Debug, Serialize)]
pub struct VerifyCodeResponse {
    pub ok: bool,
    pub user_id: i32,
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateResetTokenBody {
    pub email: String,
    pub token: String,
    pub expires_at: String,
}

#[derive(Debug, Deserialize)]
pub struct ConsumeResetTokenBody {
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct SetPasswordBody {
    pub user_id: i32,
    pub new_password: String,
}

// --- Client ---
#[derive(Debug, Serialize)]
pub struct ClientRow {
    pub id: i32,
    pub name: String,
    pub api_key: String,
    pub is_active: bool,
}

// --- Homepage ---

#[derive(Debug, Serialize, Deserialize)]
pub struct HomepageHero {
    pub title: String,
    pub subtitle: Option<String>,
    pub cta_text: Option<String>,
    pub cta_link: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct HomepageHeroBody {
    #[allow(dead_code)] // ✅ เพิ่มบรรทัดนี้ เพื่อปิด Warning
    pub section_name: Option<String>,
    pub title: Option<String>,
    pub subtitle: String,
    pub cta_text: String,
    pub cta_link: String,
    pub content: Option<String>,
}

// --- Carousel ---

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
    pub id: i32,
    pub image_url: Option<String>,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub link: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteCarouselBody {
    pub id: i32,
}