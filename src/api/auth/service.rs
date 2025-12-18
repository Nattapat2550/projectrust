use crate::config::{db::DB, env::Env};
use crate::core::errors::AppError;
use crate::core::utils::jwt;
use super::schema::*;
use bcrypt::{hash, verify, DEFAULT_COST};

// Helper Struct สำหรับรับค่าจาก DB
#[derive(sqlx::FromRow)]
struct UserRow {
    id: i32,
    username: Option<String>,
    email: String,
    password_hash: Option<String>,
    role: String,
    profile_picture_url: Option<String>,
    is_email_verified: bool,
}

pub async fn register(db: &DB, env: &Env, body: RegisterBody) -> Result<AuthResponse, AppError> {
    let email = body.email.trim().to_lowercase();
    let password_input = body.password.as_deref().unwrap_or("");
    
    // Default username จาก email ถ้าไม่ส่งมา
    let username = body.username.or_else(|| {
        email.split('@').next().map(|s| s.to_string())
    });

    if email.is_empty() {
        return Err(AppError::bad_request("email is required"));
    }

    let pw_hash = if !password_input.is_empty() {
        Some(hash(password_input, DEFAULT_COST).map_err(|_| AppError::internal("Hash error"))?)
    } else {
        None
    };

    let result = sqlx::query_as::<_, UserRow>(
        r#"
        INSERT INTO users (username, email, password_hash, role, is_email_verified)
        VALUES ($1, $2, $3, 'user', FALSE)
        RETURNING id, username, email, password_hash, role, profile_picture_url, is_email_verified
        "#,
    )
    .bind(username)
    .bind(&email)
    .bind(pw_hash)
    .fetch_one(&db.pool)
    .await;

    let u = match result {
        Ok(row) => row,
        Err(sqlx::Error::Database(db_err)) => {
            if let Some(constraint) = db_err.constraint() {
                if constraint == "users_email_key" || constraint == "users_username_key" {
                    return Err(AppError::conflict("EMAIL_EXISTS", "Email or Username already exists"));
                }
            }
            return Err(AppError::DatabaseError(sqlx::Error::Database(db_err)));
        }
        Err(e) => return Err(AppError::DatabaseError(e)),
    };

    let token = jwt::sign(u.id, u.email.clone(), u.username.clone().unwrap_or_default(), u.role.clone(), env)
        .map_err(|_| AppError::internal("Token sign error"))?;

    Ok(AuthResponse {
        token,
        user: UserResponse {
            id: u.id,
            email: u.email,
            username: u.username,
            role: u.role,
            profile_picture_url: u.profile_picture_url,
            is_email_verified: u.is_email_verified,
        },
    })
}

pub async fn login(db: &DB, env: &Env, body: LoginBody) -> Result<AuthResponse, AppError> {
    let email = body.email.trim().to_lowercase();

    let u = sqlx::query_as::<_, UserRow>(
        r#"
        SELECT id, username, email, password_hash, role, profile_picture_url, is_email_verified
        FROM users
        WHERE email = $1
        "#,
    )
    .bind(&email)
    .fetch_optional(&db.pool)
    .await?
    .ok_or_else(|| AppError::unauthorized("INVALID_CREDENTIALS", "Invalid credentials"))?;

    let is_valid = match &u.password_hash {
        Some(hash) => verify(&body.password, hash).unwrap_or(false),
        None => false,
    };

    if !is_valid {
        return Err(AppError::unauthorized("INVALID_CREDENTIALS", "Invalid credentials"));
    }

    let token = jwt::sign(u.id, u.email.clone(), u.username.clone().unwrap_or_default(), u.role.clone(), env)
        .map_err(|_| AppError::internal("Token sign error"))?;

    Ok(AuthResponse {
        token,
        user: UserResponse {
            id: u.id,
            email: u.email,
            username: u.username,
            role: u.role,
            profile_picture_url: u.profile_picture_url,
            is_email_verified: u.is_email_verified,
        },
    })
}

pub async fn google_oauth(db: &DB, env: &Env, body: GoogleOAuthBody) -> Result<AuthResponse, AppError> {
    let email = body.email.to_lowercase();
    let provider = "google";
    let oauth_id = body.oauth_id;

    // 1. Try by OAuth
    let mut u = sqlx::query_as::<_, UserRow>(
        r#"
        SELECT id, username, email, password_hash, role, profile_picture_url, is_email_verified
        FROM users
        WHERE oauth_provider = $1 AND oauth_id = $2
        "#,
    )
    .bind(provider)
    .bind(&oauth_id)
    .fetch_optional(&db.pool)
    .await?;

    if let Some(user) = u {
        // Update logic (Found by OAuth ID)
        u = Some(sqlx::query_as::<_, UserRow>(
            r#"
            UPDATE users SET
                email = COALESCE($2, email),
                is_email_verified = TRUE,
                profile_picture_url = COALESCE($3, profile_picture_url),
                username = COALESCE(username, $4)
            WHERE id = $1
            RETURNING id, username, email, password_hash, role, profile_picture_url, is_email_verified
            "#
        )
        .bind(user.id).bind(&email).bind(&body.picture_url).bind(&body.username)
        .fetch_one(&db.pool).await?);
    } else {
        // 2. Try by Email or Create New
        let username = body.username.unwrap_or_else(|| email.split('@').next().unwrap_or("user").to_string());
        
        // แก้ไข: เพิ่ม Logic การอัปเดต profile_picture_url และ username ในกรณี ON CONFLICT (เจอ Email ซ้ำ)
        // เพื่อให้ตรงกับ pure-api: 
        // - profile_picture_url = COALESCE($4, profile_picture_url) -> ถ้ามีรูปใหม่มา ให้ทับของเดิม
        // - username = COALESCE(username, $5) -> ถ้าของเดิมไม่มี username ให้ใช้ของใหม่
        u = Some(sqlx::query_as::<_, UserRow>(
            r#"
            INSERT INTO users (username, email, password_hash, role, is_email_verified, oauth_provider, oauth_id, profile_picture_url)
            VALUES ($1, $2, NULL, 'user', TRUE, $3, $4, $5)
            ON CONFLICT (email) DO UPDATE SET 
                oauth_provider = EXCLUDED.oauth_provider, 
                oauth_id = EXCLUDED.oauth_id, 
                is_email_verified = TRUE,
                profile_picture_url = COALESCE(EXCLUDED.profile_picture_url, users.profile_picture_url),
                username = COALESCE(users.username, EXCLUDED.username)
            RETURNING id, username, email, password_hash, role, profile_picture_url, is_email_verified
            "#
        )
        .bind(username).bind(&email).bind(provider).bind(&oauth_id).bind(&body.picture_url)
        .fetch_one(&db.pool).await?);
    }

    let u = u.ok_or_else(|| AppError::unauthorized("OAUTH_FAILED", "OAuth failed"))?;
    let token = jwt::sign(u.id, u.email.clone(), u.username.clone().unwrap_or_default(), u.role.clone(), env)
        .map_err(|_| AppError::internal("Token sign error"))?;

    Ok(AuthResponse {
        token,
        user: UserResponse {
            id: u.id, email: u.email, username: u.username, role: u.role,
            profile_picture_url: u.profile_picture_url, is_email_verified: u.is_email_verified,
        },
    })
}