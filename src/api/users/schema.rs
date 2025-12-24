use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRow {
    pub id: i32,
    pub email: String,
    pub username: Option<String>,
    pub role: String,
    pub provider: Option<String>,
    pub is_verified: bool,
}

// ✅ pure-api1 compatibility: /api/users/me response shape
#[derive(Debug, Serialize, Deserialize)]
pub struct UserMeRow {
    pub id: i32,
    pub username: Option<String>,
    pub email: String,
    pub role: String,
    pub profile_picture_url: Option<String>,
    pub is_email_verified: bool,
}

// ✅ pure-api1 compatibility: PATCH /api/users/me body
#[derive(Debug, Deserialize)]
pub struct UpdateMeBody {
    pub username: Option<String>,
    pub profile_picture_url: Option<String>,
}

// ✅ ใช้สำหรับรับค่า JSON ตอนเปลี่ยน Role ในหน้า Admin
#[derive(Debug, Deserialize)]
pub struct UpdateRoleBody {
    pub role: String,
}
