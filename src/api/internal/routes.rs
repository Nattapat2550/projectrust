use axum::{routing::{get, post, put}, Router};
use crate::config::db::DB;
use super::controller;

pub fn routes(db: DB) -> Router {
    Router::new()
        // --- User/Auth (Node.js calls) ---
        .route("/find-user", post(controller::find_user))
        .route("/create-user-email", post(controller::create_user_email))
        .route("/set-oauth-user", post(controller::set_oauth_user))
        .route("/store-verification-code", post(controller::store_verification_code))
        .route("/verify-code", post(controller::verify_code))
        .route("/set-username-password", post(controller::set_username_password))
        .route("/create-reset-token", post(controller::create_reset_token))
        .route("/consume-reset-token", post(controller::consume_reset_token))
        .route("/set-password", post(controller::set_password))
        
        // --- Admin ---
        .route("/admin/users/update", post(controller::update_user))
        .route("/admin/users", get(controller::list_users))
        .route("/admin/clients", get(controller::list_clients))
        .route("/admin/clients/:id/active", put(controller::set_client_active))
        
        // --- Homepage (Matching backend/routes/homepage.js) ---
        // ✅ แก้ไข: ใช้ชื่อฟังก์ชันใหม่ (get_homepage_content, update_homepage_content)
        .route("/homepage/list", get(controller::get_homepage_content)) 
        .route("/homepage/update", post(controller::update_homepage_content)) 

        // --- Carousel (Matching backend/routes/carousel.js & models/carousel.js) ---
        .route("/carousel/list", get(controller::get_carousel))
        .route("/carousel/create", post(controller::create_carousel))
        .route("/carousel/update", post(controller::update_carousel))
        .route("/carousel/delete", post(controller::delete_carousel))

        // --- Legacy/Direct Support ---
        .route("/verification-token/:email", get(controller::get_verification_token))
        .route("/reset-token/:email", get(controller::get_reset_token))
        
        .with_state(db)
}