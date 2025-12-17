use axum::{routing::{get, put}, Router, Extension, middleware};
use crate::config::db::DB;
use crate::core::middleware::jwt_auth::mw_jwt_auth; // เฉพาะ Admin/User ที่แก้ได้
use super::controller;

pub fn routes(db: DB) -> Router {
    Router::new()
        .route("/:section_name", get(controller::get_section))
        .route("/:section_name", put(controller::update_section).layer(middleware::from_fn(mw_jwt_auth)))
        .layer(Extension(db))
}