use axum::{routing::{get, patch}, Router};

use crate::config::db::DB;
use super::controller;

pub fn routes(db: DB) -> Router {
    Router::new()
        .route("/clients", get(controller::list_clients).post(controller::create_client))
        .route("/clients/:id", patch(controller::update_client).delete(controller::delete_client))
        .with_state(db)
}
