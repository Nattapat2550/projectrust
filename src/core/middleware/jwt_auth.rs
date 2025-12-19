use axum::{extract::Request, middleware::Next, response::Response};
use serde::{Deserialize, Serialize};
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
    let token = req.headers().get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .map(|s| s.trim().to_string());

    let token = match token {
        Some(t) if !t.is_empty() => t,
        _ => return Err(AppError::unauthorized("JWT_MISSING", "Missing Authorization Bearer token")),
    };

    let claims = jwt::verify(&token).map_err(|_| AppError::unauthorized("JWT_INVALID", "Invalid token"))?;

    let user = AuthUser {
        id: claims.sub,
        email: claims.email,
        // name: claims.name, // ❌ ลบออก
        role: claims.role,
    };

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}

pub async fn mw_require_admin(req: Request, next: Next) -> Result<Response, AppError> {
    let user = req.extensions().get::<AuthUser>().cloned()
        .ok_or_else(|| AppError::unauthorized("JWT_MISSING", "Missing auth user"))?;

    if user.role != "admin" {
        return Err(AppError::forbidden("FORBIDDEN", "Admin only"));
    }
    Ok(next.run(req).await)
}