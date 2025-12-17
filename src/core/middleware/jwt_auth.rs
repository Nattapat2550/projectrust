use axum::{
    extract::Request,
    http::header,
    middleware::Next,
    response::Response,
    Extension,
};
use crate::config::env::Env;
use crate::core::{errors::AppError, utils::jwt};

pub async fn mw_jwt_auth(
    Extension(env): Extension<Env>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or(AppError::Unauthorized("Missing Authorization header".to_string()))?;

    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::Unauthorized("Invalid token format".to_string()));
    }

    let token = &auth_header[7..];

    let claims = jwt::verify_token(token, &env.jwt_secret)
        .map_err(|_| AppError::Unauthorized("Invalid or expired token".to_string()))?;

    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}