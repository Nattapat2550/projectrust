use axum::{extract::State, Json};
use serde_json::json;
use crate::core::errors::AppError;
use super::schema::*;
use super::service;

pub async fn register(State((db, _)): State<(crate::config::db::DB, crate::config::env::Env)>, Json(body): Json<RegisterBody>) -> Result<Json<serde_json::Value>, AppError> {
    service::register(&db, body).await?;
    Ok(Json(json!({ "ok": true })))
}

pub async fn verify_code(State((db, _)): State<(crate::config::db::DB, crate::config::env::Env)>, Json(body): Json<VerifyCodeBody>) -> Result<Json<serde_json::Value>, AppError> {
    service::verify_code(&db, body).await?;
    Ok(Json(json!({ "ok": true })))
}

pub async fn complete_profile(State((db, env)): State<(crate::config::db::DB, crate::config::env::Env)>, Json(body): Json<CompleteProfileBody>) -> Result<Json<serde_json::Value>, AppError> {
    let out = service::complete_profile(&db, &env, body).await?;
    Ok(Json(json!({ "ok": true, "data": out })))
}

pub async fn login(State((db, env)): State<(crate::config::db::DB, crate::config::env::Env)>, Json(body): Json<LoginBody>) -> Result<Json<serde_json::Value>, AppError> {
    let out = service::login(&db, &env, body).await?;
    Ok(Json(json!({ "ok": true, "data": out })))
}

pub async fn logout() -> Json<serde_json::Value> { Json(json!({ "ok": true })) }

pub async fn forgot_password(State((db, _)): State<(crate::config::db::DB, crate::config::env::Env)>, Json(body): Json<ForgotPasswordBody>) -> Result<Json<serde_json::Value>, AppError> {
    service::forgot_password(&db, body).await?;
    Ok(Json(json!({ "ok": true })))
}

pub async fn reset_password(State((db, _)): State<(crate::config::db::DB, crate::config::env::Env)>, Json(body): Json<ResetPasswordBody>) -> Result<Json<serde_json::Value>, AppError> {
    service::reset_password(&db, body).await?;
    Ok(Json(json!({ "ok": true })))
}

pub async fn google_oauth(State((db, env)): State<(crate::config::db::DB, crate::config::env::Env)>, Json(body): Json<GoogleOAuthBody>) -> Result<Json<serde_json::Value>, AppError> {
    let out = service::google_oauth(&db, &env, body).await?;
    Ok(Json(json!({ "ok": true, "data": out })))
}

// ✅ แก้ไข: เรียก service::get_me แทนการ return ข้อมูลจาก token (ซึ่งไม่ครบ)
pub async fn me(
    State((db, _)): State<(crate::config::db::DB, crate::config::env::Env)>,
    req: axum::extract::Request
) -> Result<Json<serde_json::Value>, AppError> {
    let user_jwt = req.extensions().get::<crate::core::middleware::jwt_auth::AuthUser>()
        .ok_or_else(|| AppError::unauthorized("JWT_MISSING", "Missing auth user"))?;
    
    let user_data = service::get_me(&db, user_jwt.id).await?;
    Ok(Json(json!({ "ok": true, "data": user_data })))
}