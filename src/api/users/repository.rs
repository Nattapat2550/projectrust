use sqlx::Row;
use crate::config::db::DB;
use crate::core::errors::AppError;

use super::schema::{UserMeRow, UserRow};

pub async fn fetch_all_users(db: &DB) -> Result<Vec<UserRow>, AppError> {
    let rows = sqlx::query(
        r#"
        SELECT id, user_id::text AS user_id, email, tel, username, first_name, last_name, status, role, oauth_provider AS provider, is_email_verified AS is_verified 
        FROM users 
        ORDER BY id DESC
        "#
    )
    .fetch_all(&db.pool)
    .await?;

    let users = rows
        .into_iter()
        .map(|r| UserRow {
            id: r.get("id"),
            user_id: r.get("user_id"),
            email: r.get("email"),
            tel: r.get("tel"),
            username: r.get("username"),
            first_name: r.get("first_name"),
            last_name: r.get("last_name"),
            status: r.get("status"),
            role: r.get("role"),
            provider: r.get("provider"),
            is_verified: r.get("is_verified"),
        })
        .collect();

    Ok(users)
}

pub async fn update_user_role_and_status(db: &DB, id: i32, role: Option<String>, status: Option<String>) -> Result<Option<UserRow>, AppError> {
    let row = sqlx::query(
        r#"
        UPDATE users
        SET role = COALESCE($2, role),
            status = COALESCE($3, status),
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, user_id::text AS user_id, email, tel, username, first_name, last_name, status, role, oauth_provider AS provider, is_email_verified AS is_verified
        "#,
    )
    .bind(id)
    .bind(role)
    .bind(status)
    .fetch_optional(&db.pool)
    .await?;

    Ok(row.map(|r| UserRow {
        id: r.get("id"),
        user_id: r.get("user_id"),
        email: r.get("email"),
        tel: r.get("tel"),
        username: r.get("username"),
        first_name: r.get("first_name"),
        last_name: r.get("last_name"),
        status: r.get("status"),
        role: r.get("role"),
        provider: r.get("provider"),
        is_verified: r.get("is_verified"),
    }))
}

pub async fn fetch_user_by_id(db: &DB, id: i32) -> Result<Option<UserMeRow>, AppError> {
    let row = sqlx::query(
        r#"
        SELECT id, user_id::text AS user_id, username, email, tel, first_name, last_name, status, role, profile_picture_url, is_email_verified
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&db.pool)
    .await?;

    Ok(row.map(|r| UserMeRow {
        id: r.get("id"),
        user_id: r.get("user_id"),
        username: r.get("username"),
        email: r.get("email"),
        tel: r.get("tel"),
        first_name: r.get("first_name"),
        last_name: r.get("last_name"),
        status: r.get("status"),
        role: r.get("role"),
        profile_picture_url: r.get("profile_picture_url"),
        is_email_verified: r.get("is_email_verified"),
    }))
}

pub async fn update_user_profile(
    db: &DB,
    id: i32,
    username: Option<String>,
    tel: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    profile_picture_url: Option<String>,
) -> Result<Option<UserMeRow>, AppError> {
    let row = sqlx::query(
        r#"
        UPDATE users
        SET username = COALESCE($2, username),
            tel = COALESCE($3, tel),
            first_name = COALESCE($4, first_name),
            last_name = COALESCE($5, last_name),
            profile_picture_url = COALESCE($6, profile_picture_url),
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, user_id::text AS user_id, username, email, tel, first_name, last_name, status, role, profile_picture_url, is_email_verified
        "#,
    )
    .bind(id)
    .bind(username)
    .bind(tel)
    .bind(first_name)
    .bind(last_name)
    .bind(profile_picture_url)
    .fetch_optional(&db.pool)
    .await?;

    Ok(row.map(|r| UserMeRow {
        id: r.get("id"),
        user_id: r.get("user_id"),
        username: r.get("username"),
        email: r.get("email"),
        tel: r.get("tel"),
        first_name: r.get("first_name"),
        last_name: r.get("last_name"),
        status: r.get("status"),
        role: r.get("role"),
        profile_picture_url: r.get("profile_picture_url"),
        is_email_verified: r.get("is_email_verified"),
    }))
}