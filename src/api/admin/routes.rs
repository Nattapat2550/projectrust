use axum::{routing::get, Router, Json};
use serde_json::json;
use crate::config::db::DB;

pub fn routes(_db: DB) -> Router {
    Router::new().route("/dashboard", get(|| async {
        Json(json!({ "message": "Admin Dashboard" }))
    }))
}