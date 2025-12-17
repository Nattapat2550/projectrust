use axum::{extract::State, response::IntoResponse};
use crate::config::env::Env;
use crate::core::errors::AppError;

use super::service;

pub async fn download_windows(State(env): State<Env>) -> Result<impl IntoResponse, AppError> {
    service::download_file(env.download_windows_path.clone()).await
}

pub async fn download_android(State(env): State<Env>) -> Result<impl IntoResponse, AppError> {
    service::download_file(env.download_android_path.clone()).await
}
