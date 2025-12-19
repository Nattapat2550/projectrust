use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize)]
// ❌ ลบ #[serde(rename_all = "camelCase")] ออก
pub struct CarouselItem {
    pub id: i32,
    pub item_index: i32,          // ✅ เพิ่ม field นี้ (Pure API มี)
    pub image_dataurl: String,    // ✅ เปลี่ยนชื่อจาก image_url เป็น image_dataurl
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub created_at: Option<NaiveDateTime>, // ✅ เพิ่ม timestamp
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCarouselBody {
    pub item_index: Option<i32>,  // ✅ เพิ่ม field นี้
    pub image_dataurl: String,    // ✅ เปลี่ยนชื่อเป็น image_dataurl (Server จะรอรับชื่อนี้)
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCarouselBody {
    pub item_index: Option<i32>,
    pub image_dataurl: Option<String>, // ✅ เปลี่ยนชื่อเป็น image_dataurl
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub description: Option<String>,
}