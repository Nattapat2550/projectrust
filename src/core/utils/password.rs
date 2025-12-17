use crate::core::errors::AppError;
use bcrypt::{hash, verify, DEFAULT_COST};

pub fn hash_password(password: &str) -> Result<String, AppError> {
    hash(password, DEFAULT_COST).map_err(|e| {
        tracing::error!("Password hash error: {}", e);
        AppError::InternalServerError
    })
}

pub fn verify_password(password: &str, hashed: &str) -> Result<bool, AppError> {
    verify(password, hashed).map_err(|e| {
        tracing::warn!("Password verify error: {}", e);
        AppError::Unauthorized("Invalid credentials".to_string())
    })
}