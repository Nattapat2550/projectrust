use serde::{Deserialize, Serialize};

// --- User Schemas ---

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")] // ✅ เพิ่มบรรทัดนี้: รองรับ oauthId, oauth_id
pub struct FindUserBody {
    pub email: Option<String>,
    pub id: Option<i32>,
    pub provider: Option<String>,
    pub oauth_id: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")] // ✅ เพิ่มบรรทัดนี้: ส่งกลับเป็น camelCase
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
#[serde(rename_all = "camelCase")]
pub struct CreateUserEmailBody {
    pub email: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")] // ✅ เพิ่มบรรทัดนี้: แก้ Error set-oauth-user
pub struct SetOAuthUserBody {
    pub email: String,
    pub provider: String,
    pub oauth_id: String,
    pub picture_url: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetUsernamePasswordBody {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserBody {
    pub id: i32,
    pub username: Option<String>,
    pub profile_picture_url: Option<String>, // Node.js admin ส่งมาเป็น snake_case หรือ camelCase ก็ได้ถ้าใส่ rename_all (แต่ตัวนี้ backend เดิม map ให้แล้ว)
}

// --- Verification & Reset ---

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")] // ✅ เพิ่มบรรทัดนี้: แก้ Error store-verification-code (userId -> user_id)
pub struct StoreVerificationCodeBody {
    pub user_id: i32,
    pub code: String,
    pub expires_at: String, // ISO string
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyCodeBody {
    pub email: String,
    pub code: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")] // ✅ เพิ่มบรรทัดนี้: ส่ง userId กลับไปให้ Node
pub struct VerifyCodeResponse {
    pub ok: bool,
    pub user_id: i32,
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateResetTokenBody {
    pub email: String,
    pub token: String,
    pub expires_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConsumeResetTokenBody {
    pub token: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")] // ✅ เพิ่มบรรทัดนี้: แก้เรื่อง setPassword
pub struct SetPasswordBody {
    pub user_id: i32,
    pub new_password: String,
}

// --- Client ---
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientRow {
    pub id: i32,
    pub name: String,
    pub api_key: String,
    pub is_active: bool,
}

// --- Homepage ---

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HomepageHero {
    pub title: String,
    pub subtitle: Option<String>,
    pub cta_text: Option<String>,
    pub cta_link: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct HomepageHeroBody {
    #[allow(dead_code)]
    pub section_name: Option<String>,
    pub title: Option<String>,
    pub subtitle: String,
    pub cta_text: String,
    pub cta_link: String,
    pub content: Option<String>,
}

// --- Carousel ---

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CarouselItem {
    pub id: i32,
    pub image_url: String,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub link: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCarouselBody {
    pub image_url: String,
    pub title: String,
    pub subtitle: String,
    pub link: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
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