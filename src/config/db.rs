use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

#[derive(Clone)]
pub struct DB {
    pub pool: PgPool,
}

impl DB {
    pub async fn connect(database_url: &str) -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(20)
            .min_connections(5)
            .acquire_timeout(Duration::from_secs(30))
            .connect(database_url)
            .await
            .expect("Failed to connect to Postgres Database");

        tracing::info!("✅ Database connected successfully");
        Self { pool }
    }
}