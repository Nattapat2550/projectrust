use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct RegisterPayload {
    #[validate(length(min = 3, message = "Username must be at least 3 chars"))]
    pub username: String,

    #[validate(email(message = "Invalid email format"))]
    pub email: String, // เพิ่ม Email

    #[validate(length(min = 6, message = "Password must be at least 6 chars"))]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct LoginPayload {
    // รองรับทั้ง username หรือ email ในช่องเดียว
    #[validate(length(min = 1, message = "Username or Email is required"))]
    pub username_or_email: String,
    
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: Option<String>,
    pub email: String,
    pub role: String,
    pub profile_picture_url: Option<String>,
}