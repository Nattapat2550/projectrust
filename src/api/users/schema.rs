use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRow {
    pub id: i32,
    pub email: String,
    pub username: Option<String>,
    pub role: String,
    pub provider: String,
    pub is_verified: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRoleBody {
    pub role: String,
}
