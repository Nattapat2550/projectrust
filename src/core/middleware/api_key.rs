use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
    Extension,
};
use crate::config::env::Env;
use crate::core::errors::AppError;

pub async fn mw_api_key_auth(
    Extension(env): Extension<Env>,
    req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let key = req
        .headers()
        .get("x-api-key")
        .and_then(|val| val.to_str().ok());

    match key {
        Some(k) if k == env.api_key => Ok(next.run(req).await),
        _ => Err(AppError::Unauthorized("Invalid API Key".to_string())),
    }
}