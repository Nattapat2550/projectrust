use axum::{routing::{get, put}, Router, middleware};

use crate::config::db::DB;
use crate::core::middleware::jwt_auth;

use super::controller;

pub fn routes(db: DB) -> Router {
    Router::new()
        .route("/", get(controller::list_users))
        .route("/:id/role", put(controller::update_role))
        // behavior ใกล้ pure-api1: users routes มักต้อง auth (ถ้าคุณไม่ต้องการ ให้เอา 2 บรรทัดนี้ออก)
        .route_layer(middleware::from_fn(jwt_auth::mw_jwt_auth))
        .with_state(db)
}
