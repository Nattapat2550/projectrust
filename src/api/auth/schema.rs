use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterBody {
    pub email: String,
    pub password: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginBody {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleOAuthBody {
    /// id_token จาก Google (ฝั่ง client)
    pub id_token: String,
    /// ถ้าฝั่ง android ส่ง access_token มา (optional)
    pub access_token: Option<String>,
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
    pub name: String,
    pub role: String,
    pub provider: String,
    pub is_verified: bool,
}
