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
            .unwrap_or(8080);

        let database_url = env::var("DATABASE_URL").unwrap_or_default();

        let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "change_me".into());
        let jwt_expires_in = env::var("JWT_EXPIRES_IN").unwrap_or_else(|_| "7d".into());

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

        let download_windows_path =
            env::var("DOWNLOAD_WINDOWS_PATH").unwrap_or_else(|_| "./downloads/MyAppSetup.exe".into());
        let download_android_path =
            env::var("DOWNLOAD_ANDROID_PATH").unwrap_or_else(|_| "./downloads/MyApp.apk".into());

        let loaded = Env {
            port,
            database_url,
            jwt_secret,
            jwt_expires_in,
            google_client_id,
            google_client_secret,
            google_redirect_uri,
            allowed_origins,
            download_windows_path,
            download_android_path,
        };

        // เก็บ global (jwt::verify ใช้)
        let _ = ENV.set(loaded.clone());
        loaded
    }
}
