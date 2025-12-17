use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct UserDto {
    pub id: i32,
    pub username: String,
    pub role: String,
    pub created_at: Option<chrono::NaiveDateTime>, // หรือใช้ String ก็ได้แล้วแต่ DB Driver
}