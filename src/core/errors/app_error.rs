use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

/// Error format:
/// { ok: false, error: { code, message, details? } }
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("{message}")]
    Http {
        status: StatusCode,
        code: String,
        message: String,
        details: Option<serde_json::Value>,
    },
}

impl AppError {
    pub fn new(status: StatusCode, code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Http {
            status,
            code: code.into(),
            message: message.into(),
            details: None,
        }
    }

    pub fn unauthorized(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, code, message)
    }

    pub fn forbidden(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(StatusCode::FORBIDDEN, code, message)
    }

    pub fn not_found(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(StatusCode::NOT_FOUND, code, message)
    }

    pub fn conflict(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(StatusCode::CONFLICT, code, message)
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, "BAD_REQUEST", message)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::DatabaseError(e) => {
                let body = Json(json!({
                    "ok": false,
                    "error": {
                        "code": "DB_ERROR",
                        "message": "Database error",
                        "details": e.to_string()
                    }
                }));
                (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
            }
            AppError::Http {
                status,
                code,
                message,
                details,
            } => {
                let body = Json(json!({
                    "ok": false,
                    "error": {
                        "code": code,
                        "message": message,
                        "details": details
                    }
                }));
                (status, body).into_response()
            }
        }
    }
}
