use axum::{middleware, routing::{get, patch}, Router};

use crate::config::db::DB;
use crate::core::middleware::jwt_auth;

use super::controller;

pub fn routes(db: DB) -> Router {
    Router::new()
        .route("/clients", get(controller::list_clients).post(controller::create_client))
        .route("/clients/:id", patch(controller::update_client).delete(controller::delete_client))
        // Admin only (pure-api1)
        .route_layer(middleware::from_fn(jwt_auth::mw_require_admin))
        .route_layer(middleware::from_fn(jwt_auth::mw_jwt_auth))
        .with_state(db)
}
