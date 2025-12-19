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
        .route_layer(middleware::from_fn(jwt_auth::mw_require_admin)) // ✅ บังคับ Admin เท่านั้น
        .route_layer(middleware::from_fn(jwt_auth::mw_jwt_auth)) // ✅ ต้อง Login ก่อน
        .with_state((db, env))
}