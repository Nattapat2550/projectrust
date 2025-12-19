use crate::config::{db::DB, env::Env};
use crate::core::errors::AppError;
use crate::core::utils::jwt;
use super::schema::*;
use bcrypt::{hash, verify, DEFAULT_COST};
use rand::{Rng, distributions::Alphanumeric};

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

// ✅ เพิ่มฟังก์ชันนี้สำหรับ Route /me
pub async fn get_me(db: &DB, user_id: i32) -> Result<UserResponse, AppError> {
    let u = sqlx::query_as::<_, UserRow>(
        r#"
        SELECT id, username, email, password_hash, role, profile_picture_url, is_email_verified
        FROM users WHERE id = $1
        "#
    )
    .bind(user_id)
    .fetch_optional(&db.pool)
    .await?
    .ok_or_else(|| AppError::not_found("USER_NOT_FOUND", "User not found"))?;

    Ok(UserResponse {
        id: u.id, email: u.email, username: u.username, role: u.role,
        profile_picture_url: u.profile_picture_url, is_email_verified: u.is_email_verified,
    })
}

pub async fn register(db: &DB, body: RegisterBody) -> Result<(), AppError> {
    let email = body.email.trim().to_lowercase();
    if email.is_empty() { return Err(AppError::bad_request("Email is required")); }

    let user = sqlx::query_as::<_, UserRow>("SELECT * FROM users WHERE email = $1").bind(&email).fetch_optional(&db.pool).await?;

    let user_id = if let Some(u) = user {
        if u.is_email_verified && u.password_hash.is_some() {
             return Err(AppError::conflict("EMAIL_EXISTS", "Email already registered"));
        }
        u.id
    } else {
        let row: (i32,) = sqlx::query_as("INSERT INTO users (email, role, is_email_verified) VALUES ($1, 'user', FALSE) RETURNING id").bind(&email).fetch_one(&db.pool).await?;
        row.0
    };

    let code: String = rand::thread_rng().gen_range(100000..999999).to_string();
    sqlx::query("INSERT INTO verification_codes (user_id, code, expires_at) VALUES ($1, $2, NOW() + INTERVAL '10 minutes')").bind(user_id).bind(&code).execute(&db.pool).await?;
    println!(">>> [MOCK EMAIL] To: {}, Code: {} <<<", email, code);
    Ok(())
}

pub async fn verify_code(db: &DB, body: VerifyCodeBody) -> Result<(), AppError> {
    let email = body.email.trim().to_lowercase();
    let user = sqlx::query_as::<_, UserRow>("SELECT * FROM users WHERE email = $1").bind(&email).fetch_optional(&db.pool).await?.ok_or_else(|| AppError::not_found("USER_NOT_FOUND", "User not found"))?;
    let row = sqlx::query("SELECT id FROM verification_codes WHERE user_id = $1 AND code = $2 AND expires_at > NOW()").bind(user.id).bind(&body.code).fetch_optional(&db.pool).await?;
    if row.is_none() { return Err(AppError::bad_request("Invalid or expired code")); }
    sqlx::query("UPDATE users SET is_email_verified = TRUE WHERE id = $1").bind(user.id).execute(&db.pool).await?;
    sqlx::query("DELETE FROM verification_codes WHERE user_id = $1").bind(user.id).execute(&db.pool).await?;
    Ok(())
}

pub async fn complete_profile(db: &DB, env: &Env, body: CompleteProfileBody) -> Result<AuthResponse, AppError> {
    let email = body.email.trim().to_lowercase();
    if body.password.len() < 6 { return Err(AppError::bad_request("Password too short")); }
    let pw_hash = hash(body.password, DEFAULT_COST).map_err(|_| AppError::internal("Hash error"))?;

    let u = sqlx::query_as::<_, UserRow>(
        r#"UPDATE users SET username = $2, password_hash = $3 WHERE email = $1 AND is_email_verified = TRUE RETURNING id, username, email, password_hash, role, profile_picture_url, is_email_verified"#
    ).bind(&email).bind(&body.username).bind(pw_hash).fetch_optional(&db.pool).await?
    .ok_or_else(|| AppError::unauthorized("NOT_VERIFIED", "User not verified or not found"))?;

    // ✅ แก้ไข: ลบ name ออกจาก sign
    let token = jwt::sign(u.id, u.email.clone(), u.role.clone(), env).map_err(|_| AppError::internal("Token sign error"))?;

    Ok(AuthResponse {
        token,
        user: UserResponse { id: u.id, email: u.email, username: u.username, role: u.role, profile_picture_url: u.profile_picture_url, is_email_verified: u.is_email_verified },
    })
}

pub async fn login(db: &DB, env: &Env, body: LoginBody) -> Result<AuthResponse, AppError> {
    let email = body.email.trim().to_lowercase();
    let u = sqlx::query_as::<_, UserRow>("SELECT * FROM users WHERE email = $1").bind(&email).fetch_optional(&db.pool).await?.ok_or_else(|| AppError::unauthorized("INVALID_CREDENTIALS", "Invalid credentials"))?;
    let is_valid = match &u.password_hash { Some(h) => verify(&body.password, h).unwrap_or(false), None => false };
    if !is_valid { return Err(AppError::unauthorized("INVALID_CREDENTIALS", "Invalid credentials")); }

    // ✅ แก้ไข: ลบ name ออกจาก sign
    let token = jwt::sign(u.id, u.email.clone(), u.role.clone(), env).map_err(|_| AppError::internal("Token sign error"))?;

    Ok(AuthResponse {
        token,
        user: UserResponse { id: u.id, email: u.email, username: u.username, role: u.role, profile_picture_url: u.profile_picture_url, is_email_verified: u.is_email_verified },
    })
}

pub async fn google_oauth(db: &DB, env: &Env, body: GoogleOAuthBody) -> Result<AuthResponse, AppError> {
    let email = body.email.to_lowercase();
    let provider = "google";
    let oauth_id = body.oauth_id;

    let existing_oauth = sqlx::query_as::<_, UserRow>("SELECT * FROM users WHERE oauth_provider = $1 AND oauth_id = $2").bind(provider).bind(&oauth_id).fetch_optional(&db.pool).await?;
    let u = if let Some(user) = existing_oauth {
        sqlx::query_as::<_, UserRow>(r#"UPDATE users SET email=$2, is_email_verified=TRUE, profile_picture_url=COALESCE($3, profile_picture_url), username=COALESCE(username, $4) WHERE id=$1 RETURNING *"#)
        .bind(user.id).bind(&email).bind(&body.picture_url).bind(&body.username).fetch_one(&db.pool).await?
    } else {
        let existing_email = sqlx::query_as::<_, UserRow>("SELECT * FROM users WHERE email=$1").bind(&email).fetch_optional(&db.pool).await?;
        if let Some(user) = existing_email {
             sqlx::query_as::<_, UserRow>(r#"UPDATE users SET oauth_provider=$2, oauth_id=$3, is_email_verified=TRUE, profile_picture_url=COALESCE($4, profile_picture_url), username=COALESCE(username, $5) WHERE id=$1 RETURNING *"#)
             .bind(user.id).bind(provider).bind(&oauth_id).bind(&body.picture_url).bind(&body.username).fetch_one(&db.pool).await?
        } else {
             let username = body.username.clone().unwrap_or_else(|| email.split('@').next().unwrap_or("user").to_string());
             sqlx::query_as::<_, UserRow>(r#"INSERT INTO users (username, email, role, is_email_verified, oauth_provider, oauth_id, profile_picture_url) VALUES ($1,$2,'user',TRUE,$3,$4,$5) RETURNING *"#)
             .bind(username).bind(&email).bind(provider).bind(&oauth_id).bind(&body.picture_url).fetch_one(&db.pool).await?
        }
    };

    // ✅ แก้ไข: ลบ name ออกจาก sign
    let token = jwt::sign(u.id, u.email.clone(), u.role.clone(), env).map_err(|_| AppError::internal("Token sign error"))?;

    Ok(AuthResponse {
        token,
        user: UserResponse { id: u.id, email: u.email, username: u.username, role: u.role, profile_picture_url: u.profile_picture_url, is_email_verified: u.is_email_verified },
    })
}

pub async fn forgot_password(db: &DB, body: ForgotPasswordBody) -> Result<(), AppError> {
    let email = body.email.trim().to_lowercase();
    let user = sqlx::query_as::<_, UserRow>("SELECT * FROM users WHERE email = $1").bind(&email).fetch_optional(&db.pool).await?;
    if let Some(u) = user {
        let token: String = rand::thread_rng().sample_iter(&Alphanumeric).take(32).map(char::from).collect();
        sqlx::query("INSERT INTO password_reset_tokens (user_id, token, expires_at) VALUES ($1, $2, NOW() + INTERVAL '30 minutes')").bind(u.id).bind(&token).execute(&db.pool).await?;
        println!(">>> [MOCK RESET LINK] http://localhost:PORT/reset.html?token={} <<<", token);
    }
    Ok(())
}

pub async fn reset_password(db: &DB, body: ResetPasswordBody) -> Result<(), AppError> {
    if body.new_password.len() < 6 { return Err(AppError::bad_request("Password too short")); }
    let row = sqlx::query("SELECT user_id FROM password_reset_tokens WHERE token = $1 AND is_used = FALSE AND expires_at > NOW()").bind(&body.token).fetch_optional(&db.pool).await?;
    let user_id: i32 = match row { Some(r) => sqlx::Row::get(&r, "user_id"), None => return Err(AppError::bad_request("Invalid or expired token")) };
    let pw_hash = hash(body.new_password, DEFAULT_COST).map_err(|_| AppError::internal("Hash error"))?;
    sqlx::query("UPDATE users SET password_hash = $1 WHERE id = $2").bind(pw_hash).bind(user_id).execute(&db.pool).await?;
    sqlx::query("UPDATE password_reset_tokens SET is_used = TRUE WHERE token = $1").bind(&body.token).execute(&db.pool).await?;
    Ok(())
}