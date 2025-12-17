use axum::{extract::State, Json};
use serde_json::json;

use crate::core::errors::AppError;

use super::schema::{GoogleOAuthBody, LoginBody, RegisterBody};
use super::service;

pub async fn register(
    State((db, env)): State<(crate::config::db::DB, crate::config::env::Env)>,
    Json(body): Json<RegisterBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    let out = service::register(&db, &env, body).await?;
    Ok(Json(json!({ "ok": true, "data": out })))
}

pub async fn login(
    State((db, env)): State<(crate::config::db::DB, crate::config::env::Env)>,
    Json(body): Json<LoginBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    let out = service::login(&db, &env, body).await?;
    Ok(Json(json!({ "ok": true, "data": out })))
}

pub async fn google_oauth(
    State((db, env)): State<(crate::config::db::DB, crate::config::env::Env)>,
    Json(body): Json<GoogleOAuthBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    let out = service::google_oauth(&db, &env, body).await?;
    Ok(Json(json!({ "ok": true, "data": out })))
}

pub async fn me(req: axum::extract::Request) -> Result<Json<serde_json::Value>, AppError> {
    let user = req
        .extensions()
        .get::<crate::core::middleware::jwt_auth::AuthUser>()
        .cloned()
        .ok_or_else(|| AppError::unauthorized("JWT_MISSING", "Missing auth user"))?;

    Ok(Json(json!({ "ok": true, "data": user })))
}
