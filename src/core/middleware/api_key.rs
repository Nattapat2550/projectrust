use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
    Extension,
};
use serde::{Deserialize, Serialize};
use sqlx::Row;

use crate::config::db::DB;
use crate::core::errors::AppError;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiClient {
    pub id: i32,
    pub name: String,
    pub api_key: String,
    pub is_active: bool,
}

/// Middleware: require x-api-key (เหมือน pure-api1: app.use("/api", apiKeyAuth))
pub async fn mw_api_key_auth(
    Extension(db): Extension<DB>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let key = req
        .headers()
        .get("x-api-key")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.trim().to_string());

    let key = match key {
        Some(k) if !k.is_empty() => k,
        _ => return Err(AppError::unauthorized("API_KEY_MISSING", "Missing x-api-key")),
    };

    let row = sqlx::query(
        r#"
        SELECT id, name, api_key, is_active
        FROM api_clients
        WHERE api_key = $1
        LIMIT 1
        "#,
    )
    .bind(&key)
    .fetch_optional(&db.pool)
    .await?;

    let Some(row) = row else {
        return Err(AppError::unauthorized("API_KEY_INVALID", "Invalid x-api-key"));
    };

    let is_active: bool = row.get("is_active");
    if !is_active {
        return Err(AppError::unauthorized("API_KEY_INACTIVE", "API key is inactive"));
    }

    let client = ApiClient {
        id: row.get("id"),
        name: row.get("name"),
        api_key: row.get("api_key"),
        is_active,
    };

    req.extensions_mut().insert(client);

    Ok(next.run(req).await)
}
