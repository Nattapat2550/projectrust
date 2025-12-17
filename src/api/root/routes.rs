use axum::{routing::get, Router, Json};
use serde_json::json;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(|| async {
            Json(json!({
                "name": "pure-api",
                "version": "1.0.0",
                "status": "running",
                "tech": "Rust + Axum"
            }))
        }))
        .route("/health", get(|| async {
            Json(json!({ "ok": true }))
        }))
}
