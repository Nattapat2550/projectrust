use axum::{routing::{get, delete}, Router};
use super::controller;
use crate::config::db::DB;

pub fn routes(_db: DB) -> Router {
    Router::new()
        .route("/", get(controller::get_users))
        .route("/:id", get(controller::get_user))
        .route("/:id", delete(controller::delete_user))
}