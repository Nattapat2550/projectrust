use axum::http::StatusCode;
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::core::errors::AppError;
use tokio::task;

pub async fn hash_password(password: String) -> Result<String, AppError> {
    task::spawn_blocking(move || {
        hash(&password, DEFAULT_COST).map_err(|_| {
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "HASH_ERROR", "Password hash error")
        })
    })
    .await
    .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "THREAD_ERROR", "Thread pool error"))?
}

pub async fn verify_password(password: String, password_hash: String) -> Result<bool, AppError> {
    task::spawn_blocking(move || {
        verify(&password, &password_hash).map_err(|_| {
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "VERIFY_ERROR", "Password verify error")
        })
    })
    .await
    .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "THREAD_ERROR", "Thread pool error"))?
}