use axum::{routing::get, Router, Extension};
use serde_json::json;
use crate::config::db::DB; // ลบ env::Env ออก

// แก้ตรงนี้: เปลี่ยน db เป็น _db
pub fn routes(_db: DB) -> Router {
    Router::new().route("/health", get(move |Extension(db): Extension<DB>| async move {
        let db_status = match sqlx::query("SELECT 1").execute(&db.pool).await {
            Ok(_) => "connected",
            Err(_) => "disconnected"
        };

        axum::Json(json!({
            "server": "ok",
            "database": db_status,
            "timestamp": chrono::Utc::now().to_rfc3339()
        }))
    }))
}