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

// ✅ ใช้สำหรับรับค่า JSON ตอนเปลี่ยน Role ในหน้า Admin
#[derive(Debug, Deserialize)]
pub struct UpdateRoleBody {
    pub role: String,
}