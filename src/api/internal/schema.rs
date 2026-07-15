use serde::{Deserialize, Serialize};

// --- User Schemas ---

#[derive(Debug, Deserialize)]
pub struct FindUserBody {
    pub email: Option<String>,
    pub id: Option<String>,
    pub provider: Option<String>,
    #[serde(alias = "oauthId", alias = "oauth_id")]
    pub oauth_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserLite {
    pub id: String,
    pub email: String,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub tel: Option<String>,
    pub status: Option<String>,
    pub role: String,
    pub password_hash: Option<String>,
    pub is_email_verified: bool,
    pub oauth_provider: Option<String>,
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
    #[serde(alias = "oauthId", alias = "oauth_id")]
    pub oauth_id: String,
    #[serde(alias = "pictureUrl", alias = "picture_url")]
    pub picture_url: Option<String>,
    pub name: Option<String>,
}

// เอา rename_all ออก และดักจับทุกรูปแบบ
#[derive(Debug, Deserialize)]
pub struct SetUsernamePasswordBody {
    pub email: String,
    pub username: String,
    pub password: String,
    #[serde(alias = "firstname", alias = "firstName", alias = "first_name")]
    pub first_name: Option<String>,
    #[serde(alias = "lastname", alias = "lastName", alias = "last_name")]
    pub last_name: Option<String>,
    #[serde(alias = "telephone", alias = "phone", alias = "tel")]
    pub tel: Option<String>,
}

// เอา rename_all ออก และดักจับทุกรูปแบบ
#[derive(Debug, Deserialize)]
pub struct UpdateUserBody {
    pub id: String,
    pub username: Option<String>,
    #[serde(alias = "profilePictureUrl", alias = "profile_picture_url")]
    pub profile_picture_url: Option<String>,
    
    #[serde(alias = "firstname", alias = "firstName", alias = "first_name")]
    pub first_name: Option<String>,
    
    #[serde(alias = "lastname", alias = "lastName", alias = "last_name")]
    pub last_name: Option<String>,
    
    #[serde(alias = "telephone", alias = "phone", alias = "tel")]
    pub tel: Option<String>,
    
    pub status: Option<String>,
    pub role: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteUserBody {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct StoreVerificationCodeBody {
    #[serde(alias = "userId", alias = "user_id")]
    pub user_id: String,
    pub code: String,
    #[serde(alias = "expiresAt", alias = "expires_at")]
    pub expires_at: String,
}

#[derive(Debug, Deserialize)]
pub struct VerifyCodeBody {
    pub email: String,
    pub code: String,
}

#[derive(Debug, Serialize)]
pub struct VerifyCodeResponse {
    pub ok: bool,
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateResetTokenBody {
    pub email: String,
    pub token: String,
    #[serde(alias = "expiresAt", alias = "expires_at")]
    pub expires_at: String,
}

#[derive(Debug, Deserialize)]
pub struct ConsumeResetTokenBody {
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct SetPasswordBody {
    #[serde(alias = "userId", alias = "user_id")]
    pub user_id: String,
    #[serde(alias = "newPassword", alias = "new_password")]
    pub new_password: String,
}

#[derive(Debug, Serialize)]
pub struct ClientRow {
    pub id: String,
    pub name: String,
    #[serde(rename = "apiKey")]
    pub api_key: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
}

// --- Homepage ---
#[derive(Debug, Serialize, Deserialize)]
pub struct HomepageContentRow {
    #[serde(alias = "sectionName", alias = "section_name")]
    pub section_name: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct HomepageUpdateBody {
    #[serde(alias = "sectionName", alias = "section_name")]
    pub section_name: String,
    pub content: String,
}

// --- Carousel ---
#[derive(Debug, Serialize)]
pub struct CarouselItem {
    pub id: String,
    pub item_index: i32,
    pub image_dataurl: String,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCarouselBody {
    #[serde(alias = "image_url", alias = "image_dataurl", alias = "imageDataUrl")]
    pub image_url: String,
    #[serde(alias = "itemIndex", alias = "item_index")]
    pub item_index: Option<i32>,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    #[allow(dead_code)]
    pub link: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCarouselBody {
    pub id: String,
    #[serde(alias = "image_url", alias = "image_dataurl", alias = "imageDataUrl")]
    pub image_url: Option<String>,
    #[serde(alias = "itemIndex", alias = "item_index")]
    pub item_index: Option<i32>,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    #[allow(dead_code)]
    pub link: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteCarouselBody {
    pub id: String,
}