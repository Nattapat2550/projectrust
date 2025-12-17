use axum::{routing::get, Router};

use super::controller;

pub fn routes(env: crate::config::env::Env) -> Router {
    Router::new()
        .route("/windows", get(controller::download_windows))
        .route("/android", get(controller::download_android))
        .with_state(env)
}
