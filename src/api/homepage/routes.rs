use axum::{routing::get, Router, Json};
use serde_json::json;
use crate::config::db::DB;

pub fn routes(_db: DB) -> Router {
    Router::new().route("/", get(|| async {
        Json(json!({ "message": "Homepage data placeholder" }))
    }))
}