use axum::{routing::{get, patch}, Router, middleware};

use crate::config::db::DB;
use crate::core::middleware::jwt_auth;

use super::controller;

pub fn routes(db: DB) -> Router {
    Router::new()
        .route("/", get(controller::list).post(controller::create))
        .route("/:id", patch(controller::update).delete(controller::delete))
        .route_layer(middleware::from_fn(jwt_auth::mw_require_admin))
        .route_layer(middleware::from_fn(jwt_auth::mw_jwt_auth))
        .with_state(db)
}
