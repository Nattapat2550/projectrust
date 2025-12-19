use axum::{routing::{post, get}, Router, middleware};

use crate::config::{db::DB, env::Env};
use crate::core::middleware::jwt_auth;

use super::controller;

pub fn routes(db: DB, env: Env) -> Router {
    Router::new()
        // Auth Flow
        .route("/register", post(controller::register))
        .route("/verify-code", post(controller::verify_code))
        .route("/complete-profile", post(controller::complete_profile))
        .route("/login", post(controller::login))
        .route("/logout", post(controller::logout))
        
        // Password Reset
        .route("/forgot-password", post(controller::forgot_password))
        .route("/reset-password", post(controller::reset_password))
        
        // OAuth
        .route("/oauth/google", post(controller::google_oauth))
        
        // User Info
        .route(
            "/me",
            get(controller::me)
                .route_layer(middleware::from_fn(jwt_auth::mw_jwt_auth)),
        )
        .with_state((db, env))
}