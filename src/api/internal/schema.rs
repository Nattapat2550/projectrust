use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")] // ✅ เพิ่มบรรทัดนี้เพื่อให้รับ userId, oauthId จาก Node.js ได้
pub struct FindUserBody {
    pub email: Option<String>,
    pub id: Option<i32>,
    pub provider: Option<String>,
    pub oauth_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserEmailBody {
    pub email: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetOAuthUserBody {
    pub email: String,
    pub provider: String,
    pub oauth_id: String,       // Node ส่ง oauthId -> Rust รับ oauth_id
    pub picture_url: Option<String>, // Node ส่ง pictureUrl -> Rust รับ picture_url
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoreVerificationCodeBody {
    pub user_id: i32,          // Node ส่ง userId -> Rust รับ user_id
    pub code: String,
    pub expires_at: String,    // Node ส่ง expiresAt -> Rust รับ expires_at
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyCodeBody {
    pub email: String,
    pub code: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")] // ส่งกลับไปเป็น camelCase (userId) ให้ Node.js เข้าใจ
pub struct VerifyCodeResponse {
    pub ok: bool,
    pub user_id: i32,
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetUsernamePasswordBody {
    pub email: String,
    pub username: String,
    pub password: String,
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
#[serde(rename_all = "camelCase")]
pub struct SetPasswordBody {
    pub user_id: i32,
    pub new_password: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")] // ส่งกลับหา Node.js เป็น camelCase
pub struct UserLite {
    pub id: i32,
    pub email: String,
    pub username: Option<String>,
    pub role: String,
    pub provider: Option<String>,
    pub is_verified: bool,
}

// --- ส่วนอื่นๆ เหมือนเดิม ---
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientRow { pub id: i32, pub name: String, pub api_key: String, pub is_active: bool }

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HomepageHero { pub title: String, pub subtitle: Option<String>, pub cta_text: Option<String>, pub cta_link: Option<String> }

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HomepageHeroBody { pub title: String, pub subtitle: String, pub cta_text: String, pub cta_link: String }

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CarouselItem { pub id: i32, pub image_url: String, pub title: Option<String>, pub subtitle: Option<String>, pub link: Option<String> }

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCarouselBody { pub image_url: String, pub title: String, pub subtitle: String, pub link: String }

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCarouselBody { pub image_url: Option<String>, pub title: Option<String>, pub subtitle: Option<String>, pub link: Option<String> }