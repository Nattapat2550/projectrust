use axum::{Extension, Json};
use validator::Validate;
use crate::config::{db::DB, env::Env};
use crate::core::errors::AppError;
use super::schema::{LoginPayload, RegisterPayload};
use super::service;
use serde_json::{json, Value};

pub async fn register(
    Extension(db): Extension<DB>,
    Json(payload): Json<RegisterPayload>,
) -> Result<Json<Value>, AppError> {
    // Validate Input
    if let Err(e) = payload.validate() {
        return Err(AppError::ValidationError(e.to_string()));
    }

    let user = service::register(&db, payload).await?;
    
    Ok(Json(json!({
        "status": "success",
        "message": "User registered successfully",
        "data": user
    })))
}

pub async fn login(
    Extension(db): Extension<DB>,
    Extension(env): Extension<Env>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<Value>, AppError> {
    if let Err(e) = payload.validate() {
        return Err(AppError::ValidationError(e.to_string()));
    }

    let result = service::login(&db, &env, payload).await?;

    Ok(Json(json!({
        "status": "success",
        "data": result
    })))
}