use axum::{routing::{get, put}, Router, middleware};

use crate::config::db::DB;
use crate::core::middleware::jwt_auth;

use super::controller;

pub fn routes(db: DB) -> Router {
    // ✅ pure-api1: GET public, PUT admin
    let public_routes = Router::new().route("/:section", get(controller::get_section));

    let admin_routes = Router::new()
        .route("/:section", put(controller::put_section))
        .route_layer(middleware::from_fn(jwt_auth::mw_require_admin))
        .route_layer(middleware::from_fn(jwt_auth::mw_jwt_auth));

    public_routes.merge(admin_routes).with_state(db)
}
