use axum::{middleware, routing::{get, put}, Router};

use crate::config::db::DB;
use crate::core::middleware::jwt_auth;

use super::controller;

pub fn routes(db: DB) -> Router {
    Router::new()
        // Backward-compat
        .route("/hero", get(controller::get_hero))
        .route(
            "/hero",
            put(controller::put_hero)
                .route_layer(middleware::from_fn(jwt_auth::mw_require_admin))
                .route_layer(middleware::from_fn(jwt_auth::mw_jwt_auth)),
        )
        // pure-api1 compatibility
        .route("/:section", get(controller::get_section))
        .route(
            "/:section",
            put(controller::put_section)
                .route_layer(middleware::from_fn(jwt_auth::mw_require_admin))
                .route_layer(middleware::from_fn(jwt_auth::mw_jwt_auth)),
        )
        .with_state(db)
}
