use axum::http::StatusCode;
use bcrypt::{hash, verify, DEFAULT_COST};

use crate::core::errors::AppError;

// เก็บไว้เผื่อใช้ภายหลัง แต่ไม่ให้เตือน
#[allow(dead_code)]
pub fn hash_password(password: &str) -> Result<String, AppError> {
    hash(password, DEFAULT_COST).map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "HASH_ERROR",
            "Password hash error",
        )
    })
}

#[allow(dead_code)]
pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError> {
    verify(password, password_hash).map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "VERIFY_ERROR",
            "Password verify error",
        )
    })
}
