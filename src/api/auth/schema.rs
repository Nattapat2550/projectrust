use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct RegisterPayload {
    #[validate(length(min = 3, message = "Username must be at least 3 characters"))]
    pub username: String,
    
    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,
    
    pub role: Option<String>, // ถ้าไม่ส่งมาจะเป็น "user"
}

#[derive(Deserialize, Validate)]
pub struct LoginPayload {
    #[validate(length(min = 1, message = "Username is required"))]
    pub username: String,
    
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
    pub username: String,
    pub role: String,
}