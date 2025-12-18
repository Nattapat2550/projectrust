use axum::{
    routing::{get, post, put, patch},
    Router,
};

use crate::config::db::DB;
use super::controller;

pub fn routes(db: DB) -> Router {
    Router::new()
        // Auth/User bootstrap
        .route("/find-user", post(controller::find_user))
        .route("/create-user-email", post(controller::create_user_email))
        .route("/set-oauth-user", post(controller::set_oauth_user))

        // Email verification + password set/reset
        .route("/store-verification-code", post(controller::store_verification_code))
        .route("/verify-code", post(controller::verify_code))
        .route("/set-username-password", post(controller::set_username_password))
        .route("/create-reset-token", post(controller::create_reset_token))
        .route("/consume-reset-token", post(controller::consume_reset_token))
        .route("/set-password", post(controller::set_password))

        // ✅ Node (projectdocker) ใช้ตอนกด Save Settings / อัปเดตรูปโปรไฟล์
        .route("/admin/users/update", post(controller::update_user))

        // ✅ Node (projectdocker) homepage section editor
        .route("/homepage/list", get(controller::homepage_list))
        .route("/homepage/update", post(controller::homepage_update))

        // ✅ Node (projectdocker) carousel item editor
        .route("/carousel/list", get(controller::carousel_list))
        .route("/carousel/create", post(controller::carousel_create))
        .route("/carousel/update", post(controller::carousel_update))
        .route("/carousel/delete", post(controller::carousel_delete))

        // Legacy routes (keep)
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
