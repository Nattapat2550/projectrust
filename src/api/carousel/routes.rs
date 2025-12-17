use axum::{routing::{get, post}, Router, Extension, middleware};
use crate::config::db::DB;
use crate::core::middleware::jwt_auth::mw_jwt_auth;
use super::controller;

// แก้ตรงนี้: เปลี่ยน db เป็น _db
pub fn routes(_db: DB) -> Router {
    Router::new()
        .route("/", get(controller::get_carousels))
        .route("/", post(controller::create_carousel).layer(middleware::from_fn(mw_jwt_auth)))
        .layer(Extension(_db)) 
}