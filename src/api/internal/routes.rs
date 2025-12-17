use axum::{routing::{get, post, put, patch}, Router};
use crate::config::db::DB;
use super::controller;

pub fn routes(db: DB) -> Router {
    Router::new()
        .route("/find-user", post(controller::find_user))
        .route("/create-user-email", post(controller::create_user_email)) // ✅ เพิ่ม
        .route("/verification-token/:email", get(controller::get_verification_token))
        .route("/reset-token/:email", get(controller::get_reset_token))
        .route("/admin/users", get(controller::list_users))
        .route("/admin/clients", get(controller::list_clients))
        .route("/admin/clients/:id/active", put(controller::set_client_active))
        .route("/homepage/hero", get(controller::get_homepage_hero).put(controller::put_homepage_hero))
        .route("/carousel", get(controller::get_carousel).post(controller::create_carousel))
        .route("/carousel/:id", patch(controller::update_carousel).delete(controller::delete_carousel))
        .with_state(db)
}