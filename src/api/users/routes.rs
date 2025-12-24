use axum::{middleware, routing::{get, patch}, Router};

use crate::config::{db::DB, env::Env};
use crate::core::middleware::jwt_auth;

use super::controller;

pub fn routes(db: DB, env: Env) -> Router {
    // /me (jwt only)
    let me_routes = Router::new()
        .route("/me", get(controller::get_me).patch(controller::patch_me))
        .route_layer(middleware::from_fn(jwt_auth::mw_jwt_auth))
        .with_state((db.clone(), env.clone()));

    // admin endpoints (jwt + admin)
    let admin_routes = Router::new()
        .route("/", get(controller::list_users))
        .route("/:id/role", patch(controller::update_role))
        .route_layer(middleware::from_fn(jwt_auth::mw_require_admin))
        .route_layer(middleware::from_fn(jwt_auth::mw_jwt_auth))
        .with_state((db, env));

    Router::new().merge(me_routes).merge(admin_routes)
}
