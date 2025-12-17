use std::env;
use dotenvy::dotenv;

#[derive(Clone, Debug)]
pub struct Env {
    pub port: u16,
    pub database_url: String,
    pub jwt_secret: String,

    #[allow(dead_code)] // เพิ่มบรรทัดนี้เพื่อบอก Rust ว่าไม่ต้องแจ้งเตือน
    pub jwt_expires_in: String,

    #[allow(dead_code)] // เพิ่มบรรทัดนี้เพื่อบอก Rust ว่าไม่ต้องแจ้งเตือน
    pub node_env: String,
}

impl Env {
    pub fn new() -> Self {
        dotenv().ok();

        Self {
            port: env::var("PORT")
                .unwrap_or_else(|_| "5000".to_string())
                .parse()
                .expect("PORT must be a number"),
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
            jwt_secret: env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set"),
            jwt_expires_in: env::var("JWT_EXPIRES_IN")
                .unwrap_or_else(|_| "30d".to_string()),
            node_env: env::var("NODE_ENV")
                .unwrap_or_else(|_| "development".to_string()),
        }
    }
}