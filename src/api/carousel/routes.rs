use axum::{
    middleware,
    routing::{get, post, put},
    Router,
};

use crate::config::db::DB;
use crate::core::middleware::jwt_auth;

use super::controller;

pub fn routes(db: DB) -> Router {
    // Public: list carousel
    let public_routes = Router::new().route("/", get(controller::list)).with_state(db.clone());

    // Protected: create/update/delete (admin only)
    let protected_routes = Router::new()
        .route("/", post(controller::create))
        .route(
            "/:id",
            // Node (pure-api1) uses PUT, but keep PATCH for backward-compat
            put(controller::update)
                .patch(controller::update)
                .delete(controller::delete)
                .route_layer(middleware::from_fn(jwt_auth::mw_require_admin))
                .route_layer(middleware::from_fn(jwt_auth::mw_jwt_auth)),
        )
        .with_state(db);

    Router::new().merge(public_routes).merge(protected_routes)
}