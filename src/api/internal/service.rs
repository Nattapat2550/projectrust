use sqlx::Row;
use crate::config::db::DB;
use crate::core::errors::AppError;
use super::schema::*;
use bcrypt::{hash, DEFAULT_COST};
use chrono::{DateTime, Utc};

// --- User Management ---

pub async fn find_user(db: &DB, body: FindUserBody) -> Result<UserLite, AppError> {
    let row = if let Some(email) = &body.email {
        sqlx::query("SELECT id, email, username, role, password_hash, oauth_provider, is_email_verified, profile_picture_url FROM users WHERE LOWER(email) = $1")
            .bind(email.trim().to_lowercase())
            .fetch_optional(&db.pool)
            .await?
    } else if let Some(id) = body.id {
        sqlx::query("SELECT id, email, username, role, password_hash, oauth_provider, is_email_verified, profile_picture_url FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&db.pool)
            .await?
    } else if let (Some(p), Some(oid)) = (&body.provider, &body.oauth_id) {
         sqlx::query("SELECT id, email, username, role, password_hash, oauth_provider, is_email_verified, profile_picture_url FROM users WHERE oauth_provider = $1 AND oauth_id = $2")
            .bind(p)
            .bind(oid)
            .fetch_optional(&db.pool)
            .await?
    } else {
        return Err(AppError::bad_request("Missing search criteria"));
    };

    let r = row.ok_or_else(|| AppError::not_found("USER_NOT_FOUND", "User not found"))?;

    Ok(UserLite {
        id: r.get("id"),
        email: r.get("email"),
        username: r.get("username"),
        role: r.get("role"),
        password_hash: r.get("password_hash"),
        is_email_verified: r.get("is_email_verified"),
        oauth_provider: r.get("oauth_provider"),
        profile_picture_url: r.get("profile_picture_url"),
    })
}

pub async fn create_user_email(db: &DB, body: CreateUserEmailBody) -> Result<UserLite, AppError> {
    let email = body.email.trim().to_lowercase();
    let default_username = email.split('@').next().unwrap_or("user").to_string();

    let r = sqlx::query(
        "INSERT INTO users (email, username, role, is_email_verified, oauth_provider) VALUES ($1, $2, 'user', FALSE, 'local') RETURNING id, email, username, role, password_hash, oauth_provider, is_email_verified, profile_picture_url"
    )
    .bind(&email).bind(&default_username).fetch_one(&db.pool).await
    .map_err(|e| AppError::internal(format!("DB Error: {}", e)))?;

    Ok(UserLite {
        id: r.get("id"), email: r.get("email"), username: r.get("username"), role: r.get("role"),
        password_hash: r.get("password_hash"), is_email_verified: r.get("is_email_verified"),
        oauth_provider: r.get("oauth_provider"), profile_picture_url: r.get("profile_picture_url"),
    })
}

pub async fn set_oauth_user(db: &DB, body: SetOAuthUserBody) -> Result<UserLite, AppError> {
    let email = body.email.trim().to_lowercase();
    let existing = sqlx::query("SELECT id FROM users WHERE LOWER(email) = $1").bind(&email).fetch_optional(&db.pool).await?;

    let r = if let Some(u) = existing {
        let user_id: i32 = u.get("id");
        sqlx::query(
            "UPDATE users SET oauth_provider = $2, oauth_id = $3, is_email_verified = TRUE, profile_picture_url = COALESCE($4, profile_picture_url), username = COALESCE(username, $5) WHERE id = $1 RETURNING id, email, username, role, password_hash, oauth_provider, is_email_verified, profile_picture_url"
        )
        .bind(user_id).bind(&body.provider).bind(&body.oauth_id).bind(&body.picture_url).bind(&body.name)
        .fetch_one(&db.pool).await?
    } else {
        let username = body.name.clone().unwrap_or_else(|| email.split('@').next().unwrap_or("user").to_string());
        sqlx::query(
            "INSERT INTO users (email, username, role, is_email_verified, oauth_provider, oauth_id, profile_picture_url) VALUES ($1, $2, 'user', TRUE, $3, $4, $5) RETURNING id, email, username, role, password_hash, oauth_provider, is_email_verified, profile_picture_url"
        )
        .bind(&email).bind(&username).bind(&body.provider).bind(&body.oauth_id).bind(&body.picture_url)
        .fetch_one(&db.pool).await?
    };

    Ok(UserLite {
        id: r.get("id"), email: r.get("email"), username: r.get("username"), role: r.get("role"),
        password_hash: r.get("password_hash"), is_email_verified: r.get("is_email_verified"),
        oauth_provider: r.get("oauth_provider"), profile_picture_url: r.get("profile_picture_url"),
    })
}

pub async fn set_username_password(db: &DB, body: SetUsernamePasswordBody) -> Result<UserLite, AppError> {
    let email = body.email.trim().to_lowercase();
    let hash = hash(body.password, DEFAULT_COST).map_err(|_| AppError::internal("Hash error"))?;

    let r = sqlx::query(
        "UPDATE users SET username = $2, password_hash = $3 WHERE LOWER(email) = $1 RETURNING id, email, username, role, password_hash, oauth_provider, is_email_verified, profile_picture_url"
    )
    .bind(&email).bind(&body.username).bind(hash)
    .fetch_optional(&db.pool).await?;

    let Some(r) = r else { return Err(AppError::not_found("USER_NOT_FOUND", "User not found")); };

    Ok(UserLite {
        id: r.get("id"), email: r.get("email"), username: r.get("username"), role: r.get("role"),
        password_hash: r.get("password_hash"), is_email_verified: r.get("is_email_verified"),
        oauth_provider: r.get("oauth_provider"), profile_picture_url: r.get("profile_picture_url"),
    })
}

pub async fn update_user(db: &DB, body: UpdateUserBody) -> Result<UserLite, AppError> {
    let existing = sqlx::query("SELECT id, username, profile_picture_url FROM users WHERE id = $1")
        .bind(body.id).fetch_optional(&db.pool).await?;
    let Some(existing) = existing else { return Err(AppError::not_found("USER_NOT_FOUND", "User not found")); };

    let new_username = body.username.or(existing.try_get("username").ok());
    let new_pic = body.profile_picture_url.or(existing.try_get("profile_picture_url").ok());

    let r = sqlx::query(
        "UPDATE users SET username = $2, profile_picture_url = $3 WHERE id = $1 RETURNING id, email, username, role, password_hash, oauth_provider, is_email_verified, profile_picture_url"
    )
    .bind(body.id).bind(new_username).bind(new_pic)
    .fetch_one(&db.pool).await?;

    Ok(UserLite {
        id: r.get("id"), email: r.get("email"), username: r.get("username"), role: r.get("role"),
        password_hash: r.get("password_hash"), is_email_verified: r.get("is_email_verified"),
        oauth_provider: r.get("oauth_provider"), profile_picture_url: r.get("profile_picture_url"),
    })
}

pub async fn store_verification_code(db: &DB, body: StoreVerificationCodeBody) -> Result<(), AppError> {
    let expires_at = DateTime::parse_from_rfc3339(&body.expires_at)
        .map_err(|_| AppError::bad_request("Invalid date format"))?.with_timezone(&Utc);
    sqlx::query("DELETE FROM verification_codes WHERE user_id = $1").bind(body.user_id).execute(&db.pool).await?;
    sqlx::query("INSERT INTO verification_codes (user_id, code, expires_at) VALUES ($1, $2, $3)")
        .bind(body.user_id).bind(&body.code).bind(expires_at).execute(&db.pool).await?;
    Ok(())
}

pub async fn verify_code(db: &DB, body: VerifyCodeBody) -> Result<VerifyCodeResponse, AppError> {
    let email = body.email.trim().to_lowercase();
    let user = sqlx::query("SELECT id FROM users WHERE LOWER(email) = $1").bind(&email).fetch_optional(&db.pool).await?;
    let Some(user) = user else { return Err(AppError::not_found("USER_NOT_FOUND", "User not found")); };
    let user_id: i32 = user.get("id");

    let code_row = sqlx::query("SELECT id FROM verification_codes WHERE user_id = $1 AND code = $2 AND expires_at > NOW() LIMIT 1")
        .bind(user_id).bind(&body.code).fetch_optional(&db.pool).await?;

    if let Some(c) = code_row {
        let code_id: i32 = c.get("id");
        let mut tx = db.pool.begin().await?;
        sqlx::query("DELETE FROM verification_codes WHERE id = $1").bind(code_id).execute(&mut *tx).await?;
        sqlx::query("UPDATE users SET is_email_verified = TRUE WHERE id = $1").bind(user_id).execute(&mut *tx).await?;
        tx.commit().await?;
        Ok(VerifyCodeResponse { ok: true, user_id, reason: None })
    } else {
        Ok(VerifyCodeResponse { ok: false, user_id: 0, reason: Some("Invalid or expired code".into()) })
    }
}

pub async fn create_reset_token(db: &DB, body: CreateResetTokenBody) -> Result<(), AppError> {
    let user = sqlx::query("SELECT id FROM users WHERE LOWER(email) = $1").bind(&body.email).fetch_optional(&db.pool).await?;
    if let Some(u) = user {
        let user_id: i32 = u.get("id");
        let expires_at = DateTime::parse_from_rfc3339(&body.expires_at)
            .map_err(|_| AppError::bad_request("Invalid date"))?.with_timezone(&Utc);
        sqlx::query("DELETE FROM password_reset_tokens WHERE user_id = $1").bind(user_id).execute(&db.pool).await?;
        sqlx::query("INSERT INTO password_reset_tokens (user_id, token, expires_at) VALUES ($1, $2, $3)")
            .bind(user_id).bind(&body.token).bind(expires_at).execute(&db.pool).await?;
    }
    Ok(())
}

pub async fn consume_reset_token(db: &DB, body: ConsumeResetTokenBody) -> Result<UserLite, AppError> {
    let row = sqlx::query("SELECT user_id, is_used FROM password_reset_tokens WHERE token = $1 AND expires_at > NOW() AND is_used = FALSE").bind(&body.token).fetch_optional(&db.pool).await?;
    if let Some(r) = row {
        let user_id: i32 = r.get("user_id");
        sqlx::query("UPDATE password_reset_tokens SET is_used = TRUE WHERE token = $1").bind(&body.token).execute(&db.pool).await?;
        
        let r = sqlx::query("SELECT id, email, username, role, password_hash, oauth_provider, is_email_verified, profile_picture_url FROM users WHERE id = $1").bind(user_id).fetch_one(&db.pool).await?;
         Ok(UserLite {
            id: r.get("id"), email: r.get("email"), username: r.get("username"), role: r.get("role"),
            password_hash: r.get("password_hash"), is_email_verified: r.get("is_email_verified"),
            oauth_provider: r.get("oauth_provider"), profile_picture_url: r.get("profile_picture_url"),
        })
    } else {
        Err(AppError::bad_request("Invalid or expired token"))
    }
}

pub async fn set_password(db: &DB, body: SetPasswordBody) -> Result<(), AppError> {
    let hash = hash(body.new_password, DEFAULT_COST).map_err(|_| AppError::internal("Hash error"))?;
    sqlx::query("UPDATE users SET password_hash = $2 WHERE id = $1").bind(body.user_id).bind(hash).execute(&db.pool).await?;
    Ok(())
}

pub async fn list_users(db: &DB) -> Result<Vec<UserLite>, AppError> {
    let rows = sqlx::query("SELECT id, email, username, role, password_hash, oauth_provider, is_email_verified, profile_picture_url FROM users ORDER BY id DESC").fetch_all(&db.pool).await?;
    let mut out = Vec::new();
    for r in rows { out.push(UserLite { 
        id: r.get("id"), email: r.get("email"), username: r.get("username"), role: r.get("role"),
        password_hash: r.get("password_hash"), is_email_verified: r.get("is_email_verified"),
        oauth_provider: r.get("oauth_provider"), profile_picture_url: r.get("profile_picture_url"),
    }); }
    Ok(out)
}

pub async fn list_clients(db: &DB) -> Result<Vec<ClientRow>, AppError> {
    let rows = sqlx::query("SELECT id, name, api_key, is_active FROM api_clients ORDER BY id DESC").fetch_all(&db.pool).await?;
    let mut out = Vec::new();
    for r in rows { out.push(ClientRow { id: r.get("id"), name: r.get("name"), api_key: r.get("api_key"), is_active: r.get("is_active") }); }
    Ok(out)
}
pub async fn set_client_active(db: &DB, id: i32, is_active: bool) -> Result<(), AppError> {
    let res = sqlx::query("UPDATE api_clients SET is_active = $2 WHERE id = $1").bind(id).bind(is_active).execute(&db.pool).await?;
    if res.rows_affected() == 0 { return Err(AppError::not_found("CLIENT_NOT_FOUND", "Client not found")); }
    Ok(())
}

// --- Homepage (List all content) ---

pub async fn get_homepage_content(db: &DB) -> Result<Vec<HomepageContentRow>, AppError> {
    let rows = sqlx::query("SELECT section_name, content FROM homepage_content")
        .fetch_all(&db.pool).await?;
    
    let mut out = Vec::new();
    for r in rows {
        out.push(HomepageContentRow {
            section_name: r.get("section_name"),
            content: r.get("content"),
        });
    }
    Ok(out)
}

pub async fn update_homepage_content(db: &DB, body: HomepageUpdateBody) -> Result<HomepageContentRow, AppError> {
    sqlx::query("INSERT INTO homepage_content (section_name, content, updated_at) VALUES ($1, $2, NOW()) ON CONFLICT (section_name) DO UPDATE SET content = $2, updated_at = NOW()")
        .bind(&body.section_name).bind(&body.content)
        .execute(&db.pool).await?;

    Ok(HomepageContentRow {
        section_name: body.section_name,
        content: body.content,
    })
}

// --- Carousel ---

pub async fn get_carousel(db: &DB) -> Result<Vec<CarouselItem>, AppError> {
    // ใช้ image_dataurl และ description
    let rows = sqlx::query("SELECT id, title, subtitle, description, image_dataurl FROM carousel_items ORDER BY item_index ASC, id DESC").fetch_all(&db.pool).await?;
    let mut out = Vec::new();
    for r in rows { 
        out.push(CarouselItem{
            id: r.get("id"), 
            image_dataurl: r.get("image_dataurl"), 
            title: r.try_get("title").ok(), 
            subtitle: r.try_get("subtitle").ok(), 
            description: r.try_get("description").ok()
        }); 
    }
    Ok(out)
}

pub async fn create_carousel(db: &DB, body: CreateCarouselBody) -> Result<CarouselItem, AppError> {
    // ✅ แก้ไข: รับ title/subtitle ที่เป็น Option (ถ้าไม่มีให้เป็น null หรือ empty)
    let row = sqlx::query("INSERT INTO carousel_items (image_dataurl, title, subtitle, description) VALUES ($1, $2, $3, '') RETURNING id")
        .bind(&body.image_url).bind(&body.title).bind(&body.subtitle).fetch_one(&db.pool).await?;
    
    Ok(CarouselItem{
        id: row.get("id"), 
        image_dataurl: body.image_url, 
        title: body.title, // ✅ ใส่ค่าตรงๆ (เพราะเป็น Option อยู่แล้ว)
        subtitle: body.subtitle, 
        description: Some("".into())
    })
}

pub async fn update_carousel(db: &DB, body: UpdateCarouselBody) -> Result<CarouselItem, AppError> {
    let id = body.id;
    let existing = sqlx::query("SELECT id, title, subtitle, description, image_dataurl FROM carousel_items WHERE id = $1").bind(id).fetch_optional(&db.pool).await?;
    let Some(existing) = existing else { return Err(AppError::not_found("CAROUSEL_NOT_FOUND", "Not found")); };
    
    let new_image = body.image_url.unwrap_or(existing.get("image_dataurl"));
    let new_title = body.title.or(existing.try_get("title").ok());
    let new_subtitle = body.subtitle.or(existing.try_get("subtitle").ok());
    let desc: Option<String> = existing.try_get("description").ok();
    
    let row = sqlx::query("UPDATE carousel_items SET image_dataurl=$2, title=$3, subtitle=$4 WHERE id=$1 RETURNING id")
        .bind(id).bind(&new_image).bind(&new_title).bind(&new_subtitle).fetch_one(&db.pool).await?;
    
    Ok(CarouselItem{
        id: row.get("id"), 
        image_dataurl: new_image, 
        title: new_title, 
        subtitle: new_subtitle, 
        description: desc
    })
}

pub async fn delete_carousel(db: &DB, body: DeleteCarouselBody) -> Result<(), AppError> { 
    sqlx::query("DELETE FROM carousel_items WHERE id = $1").bind(body.id).execute(&db.pool).await?; 
    Ok(()) 
}

pub async fn get_verification_token(_db: &DB, _email: String) -> Result<String, AppError> { Ok("".into()) }
pub async fn get_reset_token(_db: &DB, _email: String) -> Result<String, AppError> { Ok("".into()) }