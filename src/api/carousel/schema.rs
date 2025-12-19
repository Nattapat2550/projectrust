use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime; // หรือ DateTime<Utc> ตาม config

#[derive(Debug, Serialize, Deserialize)]
// ❌ ลบ #[serde(rename_all = "camelCase")] ออก เพื่อให้ return JSON เป็น snake_case เหมือน pure-api
pub struct CarouselItem {
    pub id: i32,
    pub item_index: i32,          // ✅ เพิ่ม field นี้
    pub image_dataurl: String,    // ✅ เปลี่ยนจาก image_url เป็น image_dataurl
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub created_at: Option<NaiveDateTime>, // ✅ เพิ่ม timestamp (pure-api มี)
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCarouselBody {
    pub item_index: Option<i32>,  // ✅ เพิ่ม field นี้
    pub image_dataurl: String,    // ✅ เปลี่ยนจาก image_url เป็น image_dataurl
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCarouselBody {
    pub item_index: Option<i32>,      // ✅ เพิ่ม field นี้
    pub image_dataurl: Option<String>,// ✅ เปลี่ยนจาก image_url เป็น image_dataurl
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub description: Option<String>,
}