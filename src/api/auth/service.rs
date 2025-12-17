use crate::config::{db::DB, env::Env};
use crate::core::errors::AppError;
use crate::core::utils::jwt;

use super::schema::*;
use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::Row;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub async fn register(db: &DB, env: &Env, body: RegisterBody) -> Result<AuthResponse, AppError> {
    let email = body.email.trim().to_lowercase();
    
    // ✅ Handle Optional Fields: ถ้าไม่มี name ให้ใช้ชื่อจาก email, ถ้าไม่มี password ให้เป็นค่าว่าง
    let name = body.name.as_deref().unwrap_or_else(|| email.split('@').next().unwrap_or("user")).trim().to_string();
    let password_input = body.password.as_deref().unwrap_or("");

    if email.is_empty() {
        return Err(AppError::bad_request("email is required"));
    }

    // 1. Hash Password (ถ้ามี) ถ้าไม่มีให้เป็น None (NULL ใน DB)
    let pw_hash = if !password_input.is_empty() {
        Some(hash(password_input, DEFAULT_COST).map_err(|_| {
            AppError::internal("Password hash error")
        })?)
    } else {
        None
    };

    // 2. Insert into DB
    // ✅ is_verified = false (เพื่อให้สอดคล้องกับ flow ที่ต้องไปหน้า check.html)
    let result = sqlx::query(
        r#"
        INSERT INTO users (email, password_hash, name, role, provider, is_verified)
        VALUES ($1, $2, $3, 'user', 'local', false)
        RETURNING id, email, name, role, provider, is_verified
        "#,
    )
    .bind(&email)
    .bind(&pw_hash) // sqlx จะแปลง Option::None เป็น NULL ให้อัตโนมัติ
    .bind(&name)
    .fetch_one(&db.pool)
    .await;

    let row = match result {
        Ok(r) => r,
        Err(sqlx::Error::Database(db_err)) => {
            if let Some(constraint) = db_err.constraint() {
                if constraint == "users_email_key" {
                    return Err(AppError::conflict("EMAIL_EXISTS", "Email already exists"));
                }
            }
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

    // Note: Frontend อาจจะยังไม่ใช้ token นี้ทันที แต่ส่งกลับไปตาม format เดิม
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

    // ✅ เพิ่มการเช็ค: ถ้า user ไม่มี password_hash (เช่น สมัครผ่าน Google หรือ Email-only) แต่พยายาม login ด้วย password
    let pw_hash: Option<String> = row.get("password_hash");
    match pw_hash {
        Some(hash_str) => {
            let ok = verify(password, &hash_str).unwrap_or(false);
            if !ok {
                return Err(AppError::unauthorized("INVALID_CREDENTIALS", "Invalid email or password"));
            }
        },
        None => {
            // User ไม่มี password (อาจต้อง login ผ่านช่องทางอื่น)
            return Err(AppError::unauthorized("INVALID_CREDENTIALS", "Password not set for this user"));
        }
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
            VALUES ($1, NULL, $2, 'user', 'google', true)
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