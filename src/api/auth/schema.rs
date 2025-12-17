use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterBody {
    pub email: String,
    pub password: Option<String>, // Frontend ส่งมาแค่ email ดังนั้นต้องเป็น Option
    pub username: Option<String>, // เปลี่ยนจาก name เป็น username ให้ตรง pure-api
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginBody {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleOAuthBody {
    pub email: String,
    pub oauth_id: String,
    pub username: Option<String>,
    pub picture_url: Option<String>,
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
    pub username: Option<String>, // เปลี่ยนจาก name และรองรับ null
    pub role: String,
    pub profile_picture_url: Option<String>,
    pub is_email_verified: bool,
}