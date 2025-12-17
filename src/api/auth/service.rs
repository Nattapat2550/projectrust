use crate::config::{db::DB, env::Env};
use crate::core::errors::AppError;
use crate::core::utils::{jwt, password};
use super::schema::{AuthResponse, LoginPayload, RegisterPayload, UserResponse};
use sqlx::Row;

pub async fn register(db: &DB, payload: RegisterPayload) -> Result<UserResponse, AppError> {
    // 1. Hash Password
    let hashed_password = password::hash_password(&payload.password)?;
    let role = payload.role.unwrap_or_else(|| "user".to_string());

    // 2. Insert ลง Database
    let row = sqlx::query(
        "INSERT INTO users (username, password, role) VALUES ($1, $2, $3) RETURNING id, username, role"
    )
    .bind(&payload.username)
    .bind(hashed_password)
    .bind(&role)
    .fetch_optional(&db.pool)
    .await
    .map_err(|e| {
        // เช็ค Error กรณีชื่อซ้ำ (Unique Violation)
        if e.to_string().contains("duplicate key") {
            AppError::BadRequest("Username already exists".to_string())
        } else {
            AppError::DatabaseError(e)
        }
    })?
    .ok_or(AppError::InternalServerError)?;

    // 3. Return ข้อมูล User (ไม่รวม Password)
    Ok(UserResponse {
        id: row.get("id"),
        username: row.get("username"),
        role: row.get("role"),
    })
}

pub async fn login(db: &DB, env: &Env, payload: LoginPayload) -> Result<AuthResponse, AppError> {
    // 1. ค้นหา User
    let user_row = sqlx::query("SELECT id, username, password, role FROM users WHERE username = $1")
        .bind(&payload.username)
        .fetch_optional(&db.pool)
        .await
        .map_err(AppError::DatabaseError)?
        .ok_or(AppError::NotFound("User not found".to_string()))?;

    let id: i32 = user_row.get("id");
    let stored_hash: String = user_row.get("password");
    let role: String = user_row.get("role");
    let username: String = user_row.get("username");

    // 2. ตรวจสอบรหัสผ่าน
    if !password::verify_password(&payload.password, &stored_hash)? {
        return Err(AppError::Unauthorized("Invalid password".to_string()));
    }

    // 3. สร้าง JWT Token
    let token = jwt::sign_token(id, role.clone(), &env.jwt_secret)?;

    Ok(AuthResponse {
        token,
        user: UserResponse { id, username, role },
    })
}