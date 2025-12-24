use sqlx::Row;

use crate::config::db::DB;
use crate::core::errors::AppError;

use super::schema::{UpdateMeBody, UserMeRow, UserRow};

pub async fn list_users(db: &DB) -> Result<Vec<UserRow>, AppError> {
    let rows = sqlx::query(
        "SELECT id, email, username, role, oauth_provider AS provider, is_email_verified AS is_verified FROM users ORDER BY id DESC"
    )
    .fetch_all(&db.pool)
    .await?;

    let users = rows
        .into_iter()
        .map(|r| UserRow {
            id: r.get("id"),
            email: r.get("email"),
            username: r.get("username"),
            role: r.get("role"),
            provider: r.get("provider"),
            is_verified: r.get("is_verified"),
        })
        .collect();

    Ok(users)
}

/// Admin: PATCH /api/users/:id/role
pub async fn update_role(db: &DB, id: i32, role: String) -> Result<UserRow, AppError> {
    let row = sqlx::query(
        r#"
        UPDATE users
        SET role = $2, updated_at = NOW()
        WHERE id = $1
        RETURNING id, email, username, role, oauth_provider AS provider, is_email_verified AS is_verified
        "#,
    )
    .bind(id)
    .bind(role)
    .fetch_optional(&db.pool)
    .await?;

    match row {
        Some(r) => Ok(UserRow {
            id: r.get("id"),
            email: r.get("email"),
            username: r.get("username"),
            role: r.get("role"),
            provider: r.get("provider"),
            is_verified: r.get("is_verified"),
        }),
        None => Err(AppError::not_found("USER_NOT_FOUND", "User not found")),
    }
}

/// pure-api1: GET /api/users/me
pub async fn get_by_id(db: &DB, id: i32) -> Result<UserMeRow, AppError> {
    let row = sqlx::query(
        r#"
        SELECT id, username, email, role, profile_picture_url, is_email_verified
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&db.pool)
    .await?;

    match row {
        Some(r) => Ok(UserMeRow {
            id: r.get("id"),
            username: r.get("username"),
            email: r.get("email"),
            role: r.get("role"),
            profile_picture_url: r.get("profile_picture_url"),
            is_email_verified: r.get("is_email_verified"),
        }),
        None => Err(AppError::not_found("USER_NOT_FOUND", "User not found")),
    }
}

/// pure-api1: PATCH /api/users/me
pub async fn update_me(db: &DB, id: i32, body: UpdateMeBody) -> Result<UserMeRow, AppError> {
    let row = sqlx::query(
        r#"
        UPDATE users
        SET username = COALESCE($2, username),
            profile_picture_url = COALESCE($3, profile_picture_url),
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, username, email, role, profile_picture_url, is_email_verified
        "#,
    )
    .bind(id)
    .bind(body.username)
    .bind(body.profile_picture_url)
    .fetch_optional(&db.pool)
    .await?;

    match row {
        Some(r) => Ok(UserMeRow {
            id: r.get("id"),
            username: r.get("username"),
            email: r.get("email"),
            role: r.get("role"),
            profile_picture_url: r.get("profile_picture_url"),
            is_email_verified: r.get("is_email_verified"),
        }),
        None => Err(AppError::not_found("USER_NOT_FOUND", "User not found")),
    }
}
