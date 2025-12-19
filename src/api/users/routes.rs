use axum::{
    routing::{get, patch},
    Router,
    middleware,
};

use crate::config::{db::DB, env::Env};
use crate::core::middleware::jwt_auth;

use super::controller;

pub fn routes(db: DB, env: Env) -> Router {
    Router::new()
        .route("/", get(controller::list_users)) // GET /api/users
        .route("/:id/role", patch(controller::update_role)) // PATCH /api/users/:id/role
        // Middleware ทำงานจากล่างขึ้นบน (Bottom-Up)
        // 1. ตรวจสอบ JWT
        .route_layer(middleware::from_fn(jwt_auth::mw_jwt_auth)) 
        // 2. ตรวจสอบ Admin (ต้องผ่าน JWT ก่อนถึงจะเช็ค Role ได้)
        .route_layer(middleware::from_fn(jwt_auth::mw_require_admin)) 
        .with_state((db, env))
}