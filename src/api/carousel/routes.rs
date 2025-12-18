use axum::{
    middleware,
    routing::{get, patch, post},
    Router,
};

use crate::config::db::DB;
use crate::core::middleware::jwt_auth;

use super::controller;

pub fn routes(db: DB) -> Router {
    // ✅ public
    let public = Router::new()
        .route("/", get(controller::list))
        .with_state(db.clone());

    // ✅ admin-only
    let admin = Router::new()
        .route("/", post(controller::create))
        .route("/:id", patch(controller::update).delete(controller::delete))
        .route_layer(middleware::from_fn(jwt_auth::mw_require_admin))
        .route_layer(middleware::from_fn(jwt_auth::mw_jwt_auth))
        .with_state(db);

    public.merge(admin)
}
