use std::env;
use dotenvy::dotenv;

#[derive(Clone, Debug)]
pub struct Env {
    pub port: u16,
    pub database_url: String,
    pub jwt_secret: String,
    pub api_key: String,
    #[allow(dead_code)]
    pub node_env: String,
}

impl Env {
    pub fn new() -> Self {
        dotenv().ok();

        Self {
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("PORT must be a number"),
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set in .env"),
            jwt_secret: env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set in .env"),
            api_key: env::var("API_KEY")
                .unwrap_or_else(|_| "default-api-key".to_string()),
            node_env: env::var("NODE_ENV")
                .unwrap_or_else(|_| "development".to_string()),
        }
    }
}