use axum::{routing::{get, put}, Router, middleware};

use crate::config::db::DB;
use crate::core::middleware::jwt_auth;

use super::controller;

pub fn routes(db: DB) -> Router {
    Router::new()
        .route("/hero", get(controller::get_hero))
        .route(
            "/hero",
            put(controller::put_hero)
                .route_layer(middleware::from_fn(jwt_auth::mw_require_admin))
                .route_layer(middleware::from_fn(jwt_auth::mw_jwt_auth)),
        )
        .with_state(db)
}
