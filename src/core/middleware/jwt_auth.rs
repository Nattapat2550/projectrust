use axum::{extract::Request, middleware::Next, response::Response};
use serde::{Deserialize, Serialize};

use crate::config::env::Env;
use crate::core::errors::AppError;
use crate::core::utils::jwt;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: i32,
    pub email: String,
    // pub name: String, // ❌ ลบออก
    pub role: String,
}

pub async fn mw_jwt_auth(mut req: Request, next: Next) -> Result<Response, AppError> {
    // ✅ อ่าน Authorization แบบทนทานขึ้น (Bearer/bearer และช่องว่าง)
    let auth = req
        .headers()
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.trim().to_string());

    let token = match auth {
        Some(a) => {
            let a = a.trim();
            if let Some(t) = a.strip_prefix("Bearer ") {
                t.trim().to_string()
            } else if let Some(t) = a.strip_prefix("bearer ") {
                t.trim().to_string()
            } else {
                String::new()
            }
        }
        None => String::new(),
    };

    if token.is_empty() {
        return Err(AppError::unauthorized(
            "JWT_MISSING",
            "Missing Authorization Bearer token",
        ));
    }

    // ✅ สำคัญ: ดึง Env จาก Extension ที่ api/mod.rs ใส่ไว้ (.layer(Extension(env)))
    let env = req
        .extensions()
        .get::<Env>()
        .ok_or_else(|| AppError::internal("ENV_MISSING: Env extension not found"))?;

    // ✅ verify ด้วย secret ตรง ๆ (ไม่พึ่ง ENV global -> กัน panic 500)
    let claims = jwt::verify_with_secret(&token, env.jwt_secret.as_str())
        .map_err(|_| AppError::unauthorized("JWT_INVALID", "Invalid token"))?;

    let user = AuthUser {
        id: claims.sub,
        email: claims.email,
        role: claims.role,
    };

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}

pub async fn mw_require_admin(req: Request, next: Next) -> Result<Response, AppError> {
    let user = req
        .extensions()
        .get::<AuthUser>()
        .cloned()
        .ok_or_else(|| AppError::unauthorized("JWT_MISSING", "Missing auth user"))?;

    if user.role != "admin" {
        return Err(AppError::forbidden("FORBIDDEN", "Admin only"));
    }
    Ok(next.run(req).await)
}
