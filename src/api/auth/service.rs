use crate::config::{db::DB, env::Env};
use crate::core::errors::AppError;
use crate::core::utils::jwt;

use super::schema::*;
use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::Row;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// ลบ use axum::http::StatusCode; ออกแล้ว

pub async fn register(db: &DB, env: &Env, body: RegisterBody) -> Result<AuthResponse, AppError> {
    let email = body.email.trim().to_lowercase();
    let name = body.name.trim().to_string();
    let password = body.password;

    if email.is_empty() || name.is_empty() || password.is_empty() {
        return Err(AppError::bad_request("email, name, password are required"));
    }

    // 1. Hash Password
    let pw_hash = hash(password, DEFAULT_COST).map_err(|_| {
        AppError::internal("Password hash error")
    })?;

    // 2. Insert into DB (Handle Duplicate Email Error here)
    let result = sqlx::query(
        r#"
        INSERT INTO users (email, password_hash, name, role, provider, is_verified)
        VALUES ($1, $2, $3, 'user', 'local', true)
        RETURNING id, email, name, role, provider, is_verified
        "#,
    )
    .bind(&email)
    .bind(&pw_hash)
    .bind(&name)
    .fetch_one(&db.pool)
    .await;

    let row = match result {
        Ok(r) => r,
        Err(sqlx::Error::Database(db_err)) => {
            // เช็คว่า error มาจาก users_email_key หรือไม่
            if let Some(constraint) = db_err.constraint() {
                if constraint == "users_email_key" {
                    return Err(AppError::conflict("EMAIL_EXISTS", "Email already exists"));
                }
            }
            // ถ้าเป็น DB Error อื่นๆ ให้โยนกลับไปเป็น 500
            return Err(AppError::DatabaseError(sqlx::Error::Database(db_err)));
        }
        Err(e) => return Err(AppError::DatabaseError(e)),
    };

    let user = UserResponse {
        id: row.get("id"),
        email: row.get("email"),
        name: row.get("name"),
        role: row.get("role"),
        provider: row.get("provider"),
        is_verified: row.get("is_verified"),
    };

    let token = jwt::sign(
        user.id,
        user.email.clone(),
        user.name.clone(),
        user.role.clone(),
        env,
    )
    .map_err(|_| AppError::internal("Token sign error"))?;

    Ok(AuthResponse { token, user })
}

pub async fn login(db: &DB, env: &Env, body: LoginBody) -> Result<AuthResponse, AppError> {
    let email = body.email.trim().to_lowercase();
    let password = body.password;

    if email.is_empty() || password.is_empty() {
        return Err(AppError::bad_request("email and password are required"));
    }

    let row = sqlx::query(
        r#"
        SELECT id, email, password_hash, name, role, provider, is_verified
        FROM users
        WHERE LOWER(email) = $1
        LIMIT 1
        "#,
    )
    .bind(&email)
    .fetch_optional(&db.pool)
    .await?;

    let Some(row) = row else {
        return Err(AppError::unauthorized("INVALID_CREDENTIALS", "Invalid email or password"));
    };

    let pw_hash: String = row.get("password_hash");
    let ok = verify(password, &pw_hash).unwrap_or(false);
    if !ok {
        return Err(AppError::unauthorized("INVALID_CREDENTIALS", "Invalid email or password"));
    }

    let user = UserResponse {
        id: row.get("id"),
        email: row.get("email"),
        name: row.get("name"),
        role: row.get("role"),
        provider: row.get("provider"),
        is_verified: row.get("is_verified"),
    };

    let token = jwt::sign(
        user.id,
        user.email.clone(),
        user.name.clone(),
        user.role.clone(),
        env,
    )
    .map_err(|_| AppError::internal("Token sign error"))?;

    Ok(AuthResponse { token, user })
}

pub async fn google_oauth(db: &DB, env: &Env, body: GoogleOAuthBody) -> Result<AuthResponse, AppError> {
    let id_token = body.id_token.trim().to_string();
    if id_token.is_empty() {
        return Err(AppError::bad_request("id_token is required"));
    }

    let mut hasher = DefaultHasher::new();
    id_token.hash(&mut hasher);
    let h = hasher.finish(); 
    let email = format!("google_{:x}@example.com", h);
    let name = "Google User".to_string();

    let existing = sqlx::query(
        r#"
        SELECT id, email, name, role, provider, is_verified
        FROM users
        WHERE LOWER(email) = $1
        LIMIT 1
        "#,
    )
    .bind(email.to_lowercase())
    .fetch_optional(&db.pool)
    .await?;

    let row = if let Some(r) = existing {
        r
    } else {
        sqlx::query(
            r#"
            INSERT INTO users (email, password_hash, name, role, provider, is_verified)
            VALUES ($1, '', $2, 'user', 'google', true)
            RETURNING id, email, name, role, provider, is_verified
            "#,
        )
        .bind(&email)
        .bind(&name)
        .fetch_one(&db.pool)
        .await?
    };

    let user = UserResponse {
        id: row.get("id"),
        email: row.get("email"),
        name: row.get("name"),
        role: row.get("role"),
        provider: row.get("provider"),
        is_verified: row.get("is_verified"),
    };

    let token = jwt::sign(
        user.id,
        user.email.clone(),
        user.name.clone(),
        user.role.clone(),
        env,
    )
    .map_err(|_| AppError::internal("Token sign error"))?;

    Ok(AuthResponse { token, user })
}