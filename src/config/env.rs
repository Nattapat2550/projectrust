use serde::{Deserialize, Serialize};
use std::{env, sync::OnceLock};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Env {
    pub port: u16,
    pub database_url: String,

    pub jwt_secret: String,
    pub jwt_expires_in: String,

    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_redirect_uri: String,

    pub allowed_origins: Vec<String>,
    pub rate_limit_auth_max: u64, // เพิ่ม field นี้

    pub download_windows_path: String,
    pub download_android_path: String,
}

pub static ENV: OnceLock<Env> = OnceLock::new();

impl Env {
    pub fn load() -> Env {
        dotenvy::dotenv().ok();

        let port = env::var("PORT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(5000); // แก้ default เป็น 5000 ให้ตรง pure-api

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_expires_in = env::var("JWT_EXPIRES_IN").unwrap_or_else(|_| "30d".into());

        let google_client_id = env::var("GOOGLE_CLIENT_ID").unwrap_or_default();
        let google_client_secret = env::var("GOOGLE_CLIENT_SECRET").unwrap_or_default();
        let google_redirect_uri = env::var("GOOGLE_REDIRECT_URI")
            .unwrap_or_else(|_| format!("http://localhost:{}/api/auth/oauth/google/callback", port));

        let allowed_origins = env::var("ALLOWED_ORIGINS")
            .unwrap_or_default()
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();

        // Default 30 ตาม pure-api config
        let rate_limit_auth_max = env::var("RATE_LIMIT_AUTH_MAX")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(30);

        let download_windows_path =
            env::var("DOWNLOAD_WINDOWS_PATH").unwrap_or_else(|_| "./app/MyAppSetup.exe".into());
        let download_android_path =
            env::var("DOWNLOAD_ANDROID_PATH").unwrap_or_else(|_| "./app/app-release.apk".into());

        let loaded = Env {
            port,
            database_url,
            jwt_secret,
            jwt_expires_in,
            google_client_id,
            google_client_secret,
            google_redirect_uri,
            allowed_origins,
            rate_limit_auth_max,
            download_windows_path,
            download_android_path,
        };

        let _ = ENV.set(loaded.clone());
        loaded
    }
}