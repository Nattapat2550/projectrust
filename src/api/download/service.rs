use std::path::PathBuf;

use axum::{
    body::Body,
    http::{header, HeaderValue, StatusCode},
    response::Response,
};
use tokio::fs;

use crate::core::errors::AppError;

pub async fn download_file(path: String) -> Result<Response, AppError> {
    let p = PathBuf::from(path);

    if !p.exists() {
        return Err(AppError::not_found("FILE_NOT_FOUND", "File not found"));
    }

    let data = fs::read(&p).await.map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "FILE_READ_ERROR",
            "Unable to read file",
        )
    })?;

    let filename = p
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("download.bin");

    let mut res = Response::new(Body::from(data));
    *res.status_mut() = StatusCode::OK;

    res.headers_mut().insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_str(&format!("attachment; filename=\"{}\"", filename))
            .unwrap_or(HeaderValue::from_static("attachment")),
    );

    Ok(res)
}
