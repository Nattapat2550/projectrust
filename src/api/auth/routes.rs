use axum::{routing::{post, get}, Router, middleware};

use crate::config::{db::DB, env::Env};
use crate::core::middleware::jwt_auth;

use super::controller;

pub fn routes(db: DB, env: Env) -> Router {
    Router::new()
        .route("/register", post(controller::register))
        .route("/login", post(controller::login))
        .route("/oauth/google", post(controller::google_oauth))
        .route(
            "/me",
            get(controller::me)
                .route_layer(middleware::from_fn(jwt_auth::mw_jwt_auth)),
        )
        .with_state((db, env))
}
