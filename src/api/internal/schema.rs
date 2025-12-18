use serde::{Deserialize, Deserializer, Serialize};
use serde::de::Error as DeError;

fn de_i32_from_str_or_int<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    let v = serde_json::Value::deserialize(deserializer)?;
    match v {
        serde_json::Value::Number(n) => n
            .as_i64()
            .and_then(|x| i32::try_from(x).ok())
            .ok_or_else(|| DeError::custom("id out of range")),
        serde_json::Value::String(s) => s
            .parse::<i32>()
            .map_err(|_| DeError::custom("id must be an integer or integer string")),
        _ => Err(DeError::custom("id must be an integer or integer string")),
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
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
    pub oauth_id: String,

    /// รองรับทั้ง profilePictureUrl (camel) และ profile_picture_url (snake) จาก Node
    #[serde(alias = "profile_picture_url")]
    pub profile_picture_url: Option<String>,
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
pub struct UpdateUserBody {
    #[serde(deserialize_with = "de_i32_from_str_or_int")]
    pub id: i32,
    pub username: Option<String>,

    /// ✅ ตัวนี้คือสาเหตุหลักที่ projectdocker อัปโหลดรูปไม่ได้ (ส่ง snake_case)
    #[serde(alias = "profile_picture_url")]
    pub profile_picture_url: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
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
pub struct StoreVerificationCodeBody {
    pub user_id: i32,
    pub code: String,
    pub expires_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyCodeBody {
    pub email: String,
    pub code: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyCodeResponse {
    pub ok: bool,
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
#[serde(rename_all = "camelCase")]
pub struct ClientRow {
    pub id: i32,
    pub name: String,
    pub api_key: String,
    pub is_active: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetClientActiveBody {
    pub is_active: bool,
}

// -------------------- Internal Homepage (projectdocker) --------------------

#[derive(Debug, Serialize)]
pub struct HomepageContentRow {
    pub section_name: String,
    pub content: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct HomepageUpdateBody {
    pub section_name: String,
    pub content: String,
}

// -------------------- Internal Carousel (projectdocker) --------------------

#[derive(Debug, Serialize)]
pub struct CarouselRow {
    pub id: i32,
    pub item_index: i32,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub image_dataurl: String,
}

#[derive(Debug, Deserialize)]
pub struct CarouselCreateBody {
    #[serde(alias = "itemIndex")]
    pub item_index: Option<i32>,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub description: Option<String>,

    #[serde(alias = "imageDataUrl", alias = "image_data_url", alias = "imageDataurl")]
    pub image_dataurl: String,
}

#[derive(Debug, Deserialize)]
pub struct CarouselUpdateBody {
    #[serde(deserialize_with = "de_i32_from_str_or_int")]
    pub id: i32,

    #[serde(alias = "itemIndex")]
    pub item_index: Option<i32>,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub description: Option<String>,

    #[serde(alias = "imageDataUrl", alias = "image_data_url", alias = "imageDataurl")]
    pub image_dataurl: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct IdBody {
    #[serde(deserialize_with = "de_i32_from_str_or_int")]
    pub id: i32,
}

// -------------------- Legacy Homepage Hero --------------------

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HomepageHero {
    pub title: String,
    pub subtitle: String,
    pub cta_text: String,
    pub cta_link: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HomepageHeroBody {
    pub title: String,
    pub subtitle: String,
    pub cta_text: String,
    pub cta_link: String,
}

// -------------------- Legacy Carousel --------------------

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
    pub image_url: Option<String>,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub link: Option<String>,
}
