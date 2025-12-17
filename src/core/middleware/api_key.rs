use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
    Extension,
};
use crate::config::db::DB;
use crate::core::errors::AppError;

pub async fn mw_api_key_auth(
    Extension(db): Extension<DB>, // Inject DB แทน Env
    req: Request,
    next: Next,
) -> Result<Response, AppError> {
    // 1. ดึง key จาก header
    let key = req
        .headers()
        .get("x-api-key")
        .and_then(|val| val.to_str().ok())
        .ok_or(AppError::Unauthorized("Missing API Key".to_string()))?;

    // 2. เช็คใน Database
    let exists = sqlx::query(
        "SELECT 1 FROM api_clients WHERE api_key = $1 AND is_active = TRUE"
    )
    .bind(key)
    .fetch_optional(&db.pool)
    .await
    .map_err(AppError::DatabaseError)?;

    match exists {
        Some(_) => Ok(next.run(req).await),
        None => Err(AppError::Unauthorized("Invalid or inactive API Key".to_string())),
    }
}