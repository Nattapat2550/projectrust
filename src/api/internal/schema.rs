use serde::{Deserialize, Serialize};

// --- User Schemas ---

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
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
    pub password_hash: Option<String>,
    pub is_email_verified: bool,
    pub oauth_provider: Option<String>,
    pub profile_picture_url: Option<String>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserEmailBody { pub email: String }
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetOAuthUserBody {
    pub email: String, pub provider: String, pub oauth_id: String,
    pub picture_url: Option<String>, pub name: Option<String>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetUsernamePasswordBody { pub email: String, pub username: String, pub password: String }
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserBody {
    pub id: i32, 
    pub username: Option<String>,
    #[serde(alias = "profile_picture_url")]
    pub profile_picture_url: Option<String>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoreVerificationCodeBody { pub user_id: i32, pub code: String, pub expires_at: String }
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyCodeBody { pub email: String, pub code: String }
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")] 
pub struct VerifyCodeResponse { pub ok: bool, pub user_id: i32, pub reason: Option<String> }
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateResetTokenBody { pub email: String, pub token: String, pub expires_at: String }
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConsumeResetTokenBody { pub token: String }
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetPasswordBody { pub user_id: i32, pub new_password: String }
#[derive(Debug, Serialize)]
pub struct ClientRow { pub id: i32, pub name: String, pub api_key: String, pub is_active: bool }

// --- Homepage ---
#[derive(Debug, Serialize, Deserialize)]
pub struct HomepageContentRow { pub section_name: String, pub content: String }
#[derive(Debug, Deserialize)]
pub struct HomepageUpdateBody { pub section_name: String, pub content: String }


// --- Carousel (แก้ส่วนนี้) ---

#[derive(Debug, Serialize)]
pub struct CarouselItem {
    pub id: i32,
    pub item_index: i32,           // ✅ เพิ่ม item_index
    pub image_dataurl: String,     // Field นี้ตรงกับที่ Frontend Admin รอรับ
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCarouselBody {
    // ✅ เพิ่ม alias "imageDataUrl" (ตัว U ใหญ่) ให้ตรงกับที่ Node.js ส่งมา
    #[serde(alias = "image_url", alias = "image_dataurl", alias = "imageDataUrl")] 
    pub image_url: String, 
    
    pub item_index: Option<i32>,   // ✅ เพิ่ม item_index (Node.js ส่ง itemIndex)
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub description: Option<String>, // ✅ เพิ่ม description

    #[allow(dead_code)]
    pub link: Option<String>, 
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCarouselBody {
    pub id: i32,
    
    // ✅ เพิ่ม alias "imageDataUrl"
    #[serde(alias = "image_url", alias = "image_dataurl", alias = "imageDataUrl")]
    pub image_url: Option<String>,

    pub item_index: Option<i32>,   // ✅ เพิ่ม item_index
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub description: Option<String>, // ✅ เพิ่ม description

    #[allow(dead_code)]
    pub link: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteCarouselBody {
    pub id: i32,
}