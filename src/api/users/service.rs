use crate::config::db::DB;
use crate::core::errors::AppError;

use super::schema::{UpdateMeBody, UserMeRow, UserRow};
use super::repository; 

pub async fn list_users(db: &DB) -> Result<Vec<UserRow>, AppError> {
    repository::fetch_all_users(db).await
}

/// Admin: PATCH /api/users/:id/role
pub async fn update_role(db: &DB, id: i32, role: Option<String>, status: Option<String>) -> Result<UserRow, AppError> {
    let user_opt = repository::update_user_role_and_status(db, id, role, status).await?;

    match user_opt {
        Some(user) => Ok(user),
        None => Err(AppError::not_found("USER_NOT_FOUND", "User not found")),
    }
}

/// pure-api1: GET /api/users/me
pub async fn get_by_id(db: &DB, id: i32) -> Result<UserMeRow, AppError> {
    let user_opt = repository::fetch_user_by_id(db, id).await?;

    match user_opt {
        Some(user) => Ok(user),
        None => Err(AppError::not_found("USER_NOT_FOUND", "User not found")),
    }
}

/// pure-api1: PATCH /api/users/me
pub async fn update_me(db: &DB, id: i32, body: UpdateMeBody) -> Result<UserMeRow, AppError> {
    let user_opt = repository::update_user_profile(
        db, id, body.username, body.tel, body.first_name, body.last_name, body.profile_picture_url
    ).await?;

    match user_opt {
        Some(user) => Ok(user),
        None => Err(AppError::not_found("USER_NOT_FOUND", "User not found")),
    }
}