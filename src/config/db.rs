use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

#[derive(Clone)]
pub struct DB {
    pub pool: PgPool,
}

impl DB {
    // คืนค่า Result แทนการ panic เพื่อให้ main จัดการ error ได้
    pub async fn connect(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(20) // ปรับจูนตาม Spec server
            .min_connections(5)
            .acquire_timeout(Duration::from_secs(30))
            .idle_timeout(Duration::from_secs(600)) // เพิ่ม idle timeout
            .connect(database_url)
            .await?;

        tracing::info!("✅ Database connected successfully");
        Ok(Self { pool })
    }
}