use serde::{Deserialize, Serialize};

// Frontend ส่งมาแค่ email
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterBody {
    pub email: String,
}

// สำหรับหน้ากรอกรหัสยืนยัน (check.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyCodeBody {
    pub email: String,
    pub code: String,
}

// สำหรับหน้าตั้งชื่อและรหัสผ่าน (form.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct CompleteProfileBody {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginBody {
    pub email: String,
    pub password: String,
}

// รองรับ OAuth จาก Frontend (camelCase)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleOAuthBody {
    pub email: String,
    pub oauth_id: String,
    pub username: Option<String>,
    pub picture_url: Option<String>,
}

// สำหรับลืมรหัสผ่าน
#[derive(Debug, Serialize, Deserialize)]
pub struct ForgotPasswordBody {
    pub email: String,
}

// สำหรับตั้งรหัสผ่านใหม่ (reset.html)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResetPasswordBody {
    pub token: String,
    pub new_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: i32,
    pub email: String,
    pub username: Option<String>,
    pub role: String,
    pub profile_picture_url: Option<String>,
    pub is_email_verified: bool,
}