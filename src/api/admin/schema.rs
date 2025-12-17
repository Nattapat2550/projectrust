use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateClientBody {
    pub name: String,
    pub api_key: String,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateClientBody {
    pub name: Option<String>,
    pub api_key: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientRow {
    pub id: i32,
    pub name: String,
    pub api_key: String,
    pub is_active: bool,
    pub created_at: Option<chrono::NaiveDateTime>,
}
