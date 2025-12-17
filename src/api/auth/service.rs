use crate::config::{db::DB, env::Env};
use crate::core::errors::AppError;
use crate::core::utils::{jwt, password};
use super::schema::{AuthResponse, LoginPayload, RegisterPayload, UserResponse};
use sqlx::Row;

pub async fn register(db: &DB, payload: RegisterPayload) -> Result<UserResponse, AppError> {
    let hashed = password::hash_password(&payload.password)?;

    // Insert ลง users (ใช้ password_hash และ default role)
    let row = sqlx::query(
        r#"
        INSERT INTO users (username, email, password_hash, role, is_email_verified)
        VALUES ($1, $2, $3, 'user', FALSE)
        RETURNING id, username, email, role, profile_picture_url
        "#
    )
    .bind(&payload.username)
    .bind(&payload.email)
    .bind(hashed)
    .fetch_optional(&db.pool)
    .await
    .map_err(|e| {
        if e.to_string().contains("unique constraint") {
            AppError::BadRequest("Username or Email already exists".to_string())
        } else {
            AppError::DatabaseError(e)
        }
    })?
    .ok_or(AppError::InternalServerError)?;

    Ok(UserResponse {
        id: row.get("id"),
        username: row.get("username"),
        email: row.get("email"),
        role: row.get("role"),
        profile_picture_url: row.get("profile_picture_url"),
    })
}

pub async fn login(db: &DB, env: &Env, payload: LoginPayload) -> Result<AuthResponse, AppError> {
    // Query หาจาก username หรือ email
    let user = sqlx::query(
        r#"
        SELECT id, username, email, password_hash, role, profile_picture_url 
        FROM users 
        WHERE username = $1 OR email = $1
        "#
    )
    .bind(&payload.username_or_email)
    .fetch_optional(&db.pool)
    .await
    .map_err(AppError::DatabaseError)?
    .ok_or(AppError::NotFound("User not found".to_string()))?;

    let password_hash: String = user.get("password_hash"); // ตรงกับ db schema

    if !password::verify_password(&payload.password, &password_hash)? {
        return Err(AppError::Unauthorized("Invalid credentials".to_string()));
    }

    let id: i32 = user.get("id");
    let role: String = user.get("role");

    // สร้าง Token
    let token = jwt::sign_token(id, role.clone(), &env.jwt_secret)?;

    Ok(AuthResponse {
        token,
        user: UserResponse {
            id,
            username: user.get("username"),
            email: user.get("email"),
            role,
            profile_picture_url: user.get("profile_picture_url"),
        },
    })
}