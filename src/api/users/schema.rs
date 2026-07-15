use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRow {
    pub id: String,
    pub email: String,
    pub tel: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub role: String,
    pub status: String,
    pub provider: Option<String>,
    pub is_verified: bool,
    // last_login_at สามารถเพิ่มเป็น Option<chrono::DateTime<chrono::Utc>> ได้ถ้าต้องการส่งไปที่ Frontend
}

// ✅ pure-api1 compatibility: /api/users/me response shape
#[derive(Debug, Serialize, Deserialize)]
pub struct UserMeRow {
    pub id: String,
    pub username: Option<String>,
    pub email: String,
    pub tel: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub role: String,
    pub status: String,
    pub profile_picture_url: Option<String>,
    pub is_email_verified: bool,
}

// ✅ pure-api1 compatibility: PATCH /api/users/me body
#[derive(Debug, Deserialize)]
pub struct UpdateMeBody {
    pub username: Option<String>,
    pub tel: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub profile_picture_url: Option<String>,
}

// ✅ ใช้สำหรับรับค่า JSON ตอนเปลี่ยน Role ในหน้า Admin
#[derive(Debug, Deserialize)]
pub struct UpdateRoleBody {
    pub role: Option<String>,
    pub status: Option<String>, // ให้ Admin แบน (banned) หรือคืนสถานะได้
}