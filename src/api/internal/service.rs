use sqlx::Row;

use bcrypt::{hash, DEFAULT_COST};
use chrono::{DateTime, Utc};

use crate::config::db::DB;
use crate::core::errors::AppError;

use super::schema::*;

// -------------------- User Management --------------------

pub async fn find_user(db: &DB, body: FindUserBody) -> Result<UserLite, AppError> {
    if let Some(id) = body.id {
        return find_user_by_id(db, id).await;
    }
    if let Some(email) = body.email {
        return find_user_by_email(db, &email).await;
    }
    if let (Some(provider), Some(oauth_id)) = (body.provider, body.oauth_id) {
        return find_user_by_oauth(db, &provider, &oauth_id).await;
    }
    Err(AppError::bad_request("Missing search criteria"))
}

async fn find_user_by_id(db: &DB, id: i32) -> Result<UserLite, AppError> {
    let row = sqlx::query(
        "SELECT id, email, username, role, oauth_provider, is_email_verified, profile_picture_url
         FROM users
         WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&db.pool)
    .await?;

    let Some(r) = row else {
        return Ok(UserLite {
            id,
            email: "".to_string(),
            username: None,
            role: "user".to_string(),
            provider: None,
            is_verified: false,
            profile_picture_url: None,
        });
    };

    Ok(UserLite {
        id: r.get("id"),
        email: r.get("email"),
        username: r.try_get("username").ok(),
        role: r.get("role"),
        provider: r.try_get("oauth_provider").ok(),
        is_verified: r.get("is_email_verified"),
        profile_picture_url: r.try_get("profile_picture_url").ok(),
    })
}

async fn find_user_by_email(db: &DB, email: &str) -> Result<UserLite, AppError> {
    let row = sqlx::query(
        "SELECT id, email, username, role, oauth_provider, is_email_verified, profile_picture_url
         FROM users
         WHERE email = $1",
    )
    .bind(email)
    .fetch_optional(&db.pool)
    .await?;

    let Some(r) = row else {
        return Ok(UserLite {
            id: 0,
            email: email.to_string(),
            username: None,
            role: "user".to_string(),
            provider: None,
            is_verified: false,
            profile_picture_url: None,
        });
    };

    Ok(UserLite {
        id: r.get("id"),
        email: r.get("email"),
        username: r.try_get("username").ok(),
        role: r.get("role"),
        provider: r.try_get("oauth_provider").ok(),
        is_verified: r.get("is_email_verified"),
        profile_picture_url: r.try_get("profile_picture_url").ok(),
    })
}

async fn find_user_by_oauth(db: &DB, provider: &str, oauth_id: &str) -> Result<UserLite, AppError> {
    let row = sqlx::query(
        "SELECT id, email, username, role, oauth_provider, is_email_verified, profile_picture_url
         FROM users
         WHERE oauth_provider = $1 AND oauth_id = $2",
    )
    .bind(provider)
    .bind(oauth_id)
    .fetch_optional(&db.pool)
    .await?;

    let Some(r) = row else {
        return Ok(UserLite {
            id: 0,
            email: "".to_string(),
            username: None,
            role: "user".to_string(),
            provider: Some(provider.to_string()),
            is_verified: false,
            profile_picture_url: None,
        });
    };

    Ok(UserLite {
        id: r.get("id"),
        email: r.get("email"),
        username: r.try_get("username").ok(),
        role: r.get("role"),
        provider: r.try_get("oauth_provider").ok(),
        is_verified: r.get("is_email_verified"),
        profile_picture_url: r.try_get("profile_picture_url").ok(),
    })
}

pub async fn create_user_email(db: &DB, body: CreateUserEmailBody) -> Result<UserLite, AppError> {
    let email = body.email.trim().to_lowercase();

    // already exists?
    if let Ok(u) = find_user_by_email(db, &email).await {
        if u.id != 0 {
            return Ok(u);
        }
    }

    let row = sqlx::query(
        "INSERT INTO users (email, created_at, updated_at)
         VALUES ($1, NOW(), NOW())
         RETURNING id, email, username, role, oauth_provider, is_email_verified, profile_picture_url",
    )
    .bind(&email)
    .fetch_one(&db.pool)
    .await?;

    Ok(UserLite {
        id: row.get("id"),
        email: row.get("email"),
        username: row.try_get("username").ok(),
        role: row.get("role"),
        provider: row.try_get("oauth_provider").ok(),
        is_verified: row.get("is_email_verified"),
        profile_picture_url: row.try_get("profile_picture_url").ok(),
    })
}

pub async fn set_oauth_user(db: &DB, body: SetOAuthUserBody) -> Result<UserLite, AppError> {
    let email = body.email.trim().to_lowercase();

    let row = sqlx::query(
        "UPDATE users
         SET oauth_provider = $2,
             oauth_id = $3,
             profile_picture_url = COALESCE($4, profile_picture_url),
             updated_at = NOW()
         WHERE email = $1
         RETURNING id, email, username, role, oauth_provider, is_email_verified, profile_picture_url",
    )
    .bind(&email)
    .bind(&body.provider)
    .bind(&body.oauth_id)
    .bind(&body.profile_picture_url)
    .fetch_one(&db.pool)
    .await?;

    Ok(UserLite {
        id: row.get("id"),
        email: row.get("email"),
        username: row.try_get("username").ok(),
        role: row.get("role"),
        provider: row.try_get("oauth_provider").ok(),
        is_verified: row.get("is_email_verified"),
        profile_picture_url: row.try_get("profile_picture_url").ok(),
    })
}

pub async fn set_username_password(db: &DB, body: SetUsernamePasswordBody) -> Result<UserLite, AppError> {
    let email = body.email.trim().to_lowercase();
    let username = body.username.trim().to_string();

    let password_hash = hash(body.password, DEFAULT_COST)
        .map_err(|_| AppError::internal("Failed to hash password"))?;

    let row = sqlx::query(
        "UPDATE users
         SET username = $2,
             password_hash = $3,
             updated_at = NOW()
         WHERE email = $1
         RETURNING id, email, username, role, oauth_provider, is_email_verified, profile_picture_url",
    )
    .bind(&email)
    .bind(&username)
    .bind(&password_hash)
    .fetch_one(&db.pool)
    .await?;

    Ok(UserLite {
        id: row.get("id"),
        email: row.get("email"),
        username: row.try_get("username").ok(),
        role: row.get("role"),
        provider: row.try_get("oauth_provider").ok(),
        is_verified: row.get("is_email_verified"),
        profile_picture_url: row.try_get("profile_picture_url").ok(),
    })
}

/// ✅ สำคัญ: ต้องรองรับ profile_picture_url (snake_case) ที่ projectdocker ส่งมา
pub async fn update_user(db: &DB, body: UpdateUserBody) -> Result<UserLite, AppError> {
    let existing = sqlx::query(
        "SELECT id, email, username, role, oauth_provider, is_email_verified, profile_picture_url
         FROM users
         WHERE id = $1",
    )
    .bind(body.id)
    .fetch_optional(&db.pool)
    .await?;

    let Some(old) = existing else {
        return Err(AppError::not_found("USER_NOT_FOUND", "User not found"));
    };

    let new_username: Option<String> = body.username.or(old.try_get("username").ok());
    let new_pic: Option<String> = body
        .profile_picture_url
        .or(old.try_get("profile_picture_url").ok());

    let row = sqlx::query(
        "UPDATE users
         SET username = $2,
             profile_picture_url = $3,
             updated_at = NOW()
         WHERE id = $1
         RETURNING id, email, username, role, oauth_provider, is_email_verified, profile_picture_url",
    )
    .bind(body.id)
    .bind(new_username)
    .bind(new_pic)
    .fetch_one(&db.pool)
    .await?;

    Ok(UserLite {
        id: row.get("id"),
        email: row.get("email"),
        username: row.try_get("username").ok(),
        role: row.get("role"),
        provider: row.try_get("oauth_provider").ok(),
        is_verified: row.get("is_email_verified"),
        profile_picture_url: row.try_get("profile_picture_url").ok(),
    })
}

pub async fn list_users(db: &DB) -> Result<Vec<UserLite>, AppError> {
    let rows = sqlx::query(
        "SELECT id, email, username, role, oauth_provider, is_email_verified, profile_picture_url
         FROM users
         ORDER BY id ASC",
    )
    .fetch_all(&db.pool)
    .await?;

    let mut out = Vec::with_capacity(rows.len());
    for r in rows {
        out.push(UserLite {
            id: r.get("id"),
            email: r.get("email"),
            username: r.try_get("username").ok(),
            role: r.get("role"),
            provider: r.try_get("oauth_provider").ok(),
            is_verified: r.get("is_email_verified"),
            profile_picture_url: r.try_get("profile_picture_url").ok(),
        });
    }
    Ok(out)
}

// -------------------- Verification Codes --------------------

pub async fn store_verification_code(db: &DB, body: StoreVerificationCodeBody) -> Result<(), AppError> {
    sqlx::query(
        "INSERT INTO verification_codes (user_id, code, expires_at)
         VALUES ($1, $2, $3)
         ON CONFLICT (user_id)
         DO UPDATE SET code = EXCLUDED.code, expires_at = EXCLUDED.expires_at",
    )
    .bind(body.user_id)
    .bind(body.code)
    .bind(body.expires_at)
    .execute(&db.pool)
    .await?;

    Ok(())
}

pub async fn verify_code(db: &DB, body: VerifyCodeBody) -> Result<VerifyCodeResponse, AppError> {
    let email = body.email.trim().to_lowercase();

    let user = find_user_by_email(db, &email).await?;
    if user.id == 0 {
        return Ok(VerifyCodeResponse { ok: false });
    }

    let row = sqlx::query("SELECT code, expires_at FROM verification_codes WHERE user_id = $1")
        .bind(user.id)
        .fetch_optional(&db.pool)
        .await?;

    let Some(r) = row else {
        return Ok(VerifyCodeResponse { ok: false });
    };

    let stored: String = r.get("code");
    let _expires: String = r.get("expires_at");

    if stored != body.code {
        return Ok(VerifyCodeResponse { ok: false });
    }

    sqlx::query("UPDATE users SET is_email_verified = TRUE, updated_at = NOW() WHERE id = $1")
        .bind(user.id)
        .execute(&db.pool)
        .await?;

    sqlx::query("DELETE FROM verification_codes WHERE user_id = $1")
        .bind(user.id)
        .execute(&db.pool)
        .await?;

    Ok(VerifyCodeResponse { ok: true })
}

// -------------------- Password Reset Tokens --------------------

pub async fn create_reset_token(db: &DB, body: CreateResetTokenBody) -> Result<(), AppError> {
    let email = body.email.trim().to_lowercase();

    let user = find_user_by_email(db, &email).await?;
    if user.id == 0 {
        return Err(AppError::not_found("USER_NOT_FOUND", "User not found"));
    }

    sqlx::query(
        "INSERT INTO password_reset_tokens (user_id, token, expires_at)
         VALUES ($1, $2, $3)
         ON CONFLICT (user_id)
         DO UPDATE SET token = EXCLUDED.token, expires_at = EXCLUDED.expires_at",
    )
    .bind(user.id)
    .bind(body.token)
    .bind(body.expires_at)
    .execute(&db.pool)
    .await?;

    Ok(())
}

pub async fn consume_reset_token(db: &DB, body: ConsumeResetTokenBody) -> Result<bool, AppError> {
    let row = sqlx::query(
        "SELECT user_id, token, expires_at
         FROM password_reset_tokens
         WHERE token = $1",
    )
    .bind(body.token)
    .fetch_optional(&db.pool)
    .await?;

    let Some(r) = row else { return Ok(false); };

    let token: String = r.get("token");
    if token.is_empty() { return Ok(false); }

    Ok(true)
}

pub async fn set_password(db: &DB, body: SetPasswordBody) -> Result<(), AppError> {
    let password_hash = hash(body.new_password, DEFAULT_COST)
        .map_err(|_| AppError::internal("Failed to hash password"))?;

    sqlx::query("UPDATE users SET password_hash = $2, updated_at = NOW() WHERE id = $1")
        .bind(body.user_id)
        .bind(password_hash)
        .execute(&db.pool)
        .await?;

    sqlx::query("DELETE FROM password_reset_tokens WHERE user_id = $1")
        .bind(body.user_id)
        .execute(&db.pool)
        .await?;

    Ok(())
}

// -------------------- API Clients --------------------

pub async fn list_clients(db: &DB) -> Result<Vec<ClientRow>, AppError> {
    let rows = sqlx::query("SELECT id, name, api_key, is_active FROM api_clients ORDER BY id ASC")
        .fetch_all(&db.pool)
        .await?;

    let mut out = Vec::with_capacity(rows.len());
    for r in rows {
        out.push(ClientRow {
            id: r.get("id"),
            name: r.get("name"),
            api_key: r.get("api_key"),
            is_active: r.get("is_active"),
        });
    }
    Ok(out)
}

pub async fn set_client_active(db: &DB, id: i32, is_active: bool) -> Result<(), AppError> {
    let res = sqlx::query("UPDATE api_clients SET is_active = $2 WHERE id = $1")
        .bind(id)
        .bind(is_active)
        .execute(&db.pool)
        .await?;

    if res.rows_affected() == 0 {
        return Err(AppError::not_found("CLIENT_NOT_FOUND", "Client not found"));
    }
    Ok(())
}

// -------------------- Internal Homepage (projectdocker) --------------------

pub async fn homepage_list(db: &DB) -> Result<Vec<HomepageContentRow>, AppError> {
    let rows = sqlx::query(
        "SELECT section_name, content, updated_at
         FROM homepage_content
         ORDER BY section_name ASC",
    )
    .fetch_all(&db.pool)
    .await?;

    let mut out = Vec::with_capacity(rows.len());
    for r in rows {
        let updated_at: DateTime<Utc> = r.get("updated_at");
        out.push(HomepageContentRow {
            section_name: r.get("section_name"),
            content: r.get("content"),
            updated_at: updated_at.to_rfc3339(),
        });
    }
    Ok(out)
}

pub async fn homepage_update(db: &DB, body: HomepageUpdateBody) -> Result<HomepageContentRow, AppError> {
    let row = sqlx::query(
        "INSERT INTO homepage_content (section_name, content, updated_at)
         VALUES ($1, $2, NOW())
         ON CONFLICT (section_name)
         DO UPDATE SET content = EXCLUDED.content, updated_at = NOW()
         RETURNING section_name, content, updated_at",
    )
    .bind(body.section_name)
    .bind(body.content)
    .fetch_one(&db.pool)
    .await?;

    let updated_at: DateTime<Utc> = row.get("updated_at");

    Ok(HomepageContentRow {
        section_name: row.get("section_name"),
        content: row.get("content"),
        updated_at: updated_at.to_rfc3339(),
    })
}

// -------------------- Internal Carousel (projectdocker) --------------------

pub async fn carousel_list(db: &DB) -> Result<Vec<CarouselRow>, AppError> {
    let rows = sqlx::query(
        "SELECT id, item_index, title, subtitle, description, image_dataurl
         FROM carousel_items
         ORDER BY item_index ASC, id ASC",
    )
    .fetch_all(&db.pool)
    .await?;

    let mut out = Vec::with_capacity(rows.len());
    for r in rows {
        out.push(CarouselRow {
            id: r.get("id"),
            item_index: r.get("item_index"),
            title: r.try_get("title").ok(),
            subtitle: r.try_get("subtitle").ok(),
            description: r.try_get("description").ok(),
            image_dataurl: r.get("image_dataurl"),
        });
    }
    Ok(out)
}

pub async fn carousel_create(db: &DB, body: CarouselCreateBody) -> Result<CarouselRow, AppError> {
    let idx = body.item_index.unwrap_or(0);

    let row = sqlx::query(
        "INSERT INTO carousel_items (item_index, title, subtitle, description, image_dataurl, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
         RETURNING id, item_index, title, subtitle, description, image_dataurl",
    )
    .bind(idx)
    .bind(body.title)
    .bind(body.subtitle)
    .bind(body.description)
    .bind(body.image_dataurl)
    .fetch_one(&db.pool)
    .await?;

    Ok(CarouselRow {
        id: row.get("id"),
        item_index: row.get("item_index"),
        title: row.try_get("title").ok(),
        subtitle: row.try_get("subtitle").ok(),
        description: row.try_get("description").ok(),
        image_dataurl: row.get("image_dataurl"),
    })
}

pub async fn carousel_update(db: &DB, body: CarouselUpdateBody) -> Result<CarouselRow, AppError> {
    let row = sqlx::query(
        "UPDATE carousel_items
         SET item_index = COALESCE($2, item_index),
             title = COALESCE($3, title),
             subtitle = COALESCE($4, subtitle),
             description = COALESCE($5, description),
             image_dataurl = COALESCE($6, image_dataurl),
             updated_at = NOW()
         WHERE id = $1
         RETURNING id, item_index, title, subtitle, description, image_dataurl",
    )
    .bind(body.id)
    .bind(body.item_index)
    .bind(body.title)
    .bind(body.subtitle)
    .bind(body.description)
    .bind(body.image_dataurl)
    .fetch_optional(&db.pool)
    .await?;

    let Some(row) = row else {
        return Err(AppError::not_found("CAROUSEL_NOT_FOUND", "Carousel item not found"));
    };

    Ok(CarouselRow {
        id: row.get("id"),
        item_index: row.get("item_index"),
        title: row.try_get("title").ok(),
        subtitle: row.try_get("subtitle").ok(),
        description: row.try_get("description").ok(),
        image_dataurl: row.get("image_dataurl"),
    })
}

pub async fn carousel_delete(db: &DB, id: i32) -> Result<(), AppError> {
    let res = sqlx::query("DELETE FROM carousel_items WHERE id = $1")
        .bind(id)
        .execute(&db.pool)
        .await?;

    if res.rows_affected() == 0 {
        return Err(AppError::not_found("CAROUSEL_NOT_FOUND", "Carousel item not found"));
    }
    Ok(())
}

// -------------------- Legacy Homepage Hero (keep) --------------------

pub async fn get_homepage_hero(db: &DB) -> Result<HomepageHero, AppError> {
    let row = sqlx::query(
        "SELECT content FROM homepage_content WHERE section_name = 'hero' LIMIT 1",
    )
    .fetch_optional(&db.pool)
    .await?;

    let content = row.and_then(|r| r.try_get::<String, _>("content").ok()).unwrap_or_default();

    let parsed: serde_json::Value = serde_json::from_str(&content).unwrap_or_else(|_| serde_json::json!({}));
    Ok(HomepageHero {
        title: parsed.get("title").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        subtitle: parsed.get("subtitle").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        cta_text: parsed.get("ctaText").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        cta_link: parsed.get("ctaLink").and_then(|v| v.as_str()).unwrap_or("").to_string(),
    })
}

pub async fn put_homepage_hero(db: &DB, body: HomepageHeroBody) -> Result<HomepageHero, AppError> {
    let content = serde_json::json!({
        "title": body.title,
        "subtitle": body.subtitle,
        "ctaText": body.cta_text,
        "ctaLink": body.cta_link
    })
    .to_string();

    let _ = sqlx::query(
        "INSERT INTO homepage_content (section_name, content, updated_at)
         VALUES ('hero', $1, NOW())
         ON CONFLICT (section_name)
         DO UPDATE SET content = EXCLUDED.content, updated_at = NOW()",
    )
    .bind(&content)
    .execute(&db.pool)
    .await?;

    Ok(HomepageHero {
        title: serde_json::from_str::<serde_json::Value>(&content).unwrap()["title"].as_str().unwrap_or("").to_string(),
        subtitle: serde_json::from_str::<serde_json::Value>(&content).unwrap()["subtitle"].as_str().unwrap_or("").to_string(),
        cta_text: serde_json::from_str::<serde_json::Value>(&content).unwrap()["ctaText"].as_str().unwrap_or("").to_string(),
        cta_link: serde_json::from_str::<serde_json::Value>(&content).unwrap()["ctaLink"].as_str().unwrap_or("").to_string(),
    })
}

// -------------------- Legacy Carousel (keep) --------------------

pub async fn get_carousel(db: &DB) -> Result<Vec<CarouselItem>, AppError> {
    let rows = sqlx::query(
        "SELECT id, title, subtitle, image_dataurl
         FROM carousel_items
         ORDER BY id DESC",
    )
    .fetch_all(&db.pool)
    .await?;

    let mut out = Vec::with_capacity(rows.len());
    for r in rows {
        out.push(CarouselItem {
            id: r.get("id"),
            image_url: r.get("image_dataurl"),
            title: r.try_get("title").ok(),
            subtitle: r.try_get("subtitle").ok(),
            link: None,
        });
    }
    Ok(out)
}

pub async fn create_carousel(db: &DB, body: CreateCarouselBody) -> Result<CarouselItem, AppError> {
    let row = sqlx::query(
        "INSERT INTO carousel_items (image_dataurl, title, subtitle, created_at, updated_at)
         VALUES ($1, $2, $3, NOW(), NOW())
         RETURNING id",
    )
    .bind(&body.image_url)
    .bind(&body.title)
    .bind(&body.subtitle)
    .fetch_one(&db.pool)
    .await?;

    Ok(CarouselItem {
        id: row.get("id"),
        image_url: body.image_url,
        title: Some(body.title),
        subtitle: Some(body.subtitle),
        link: Some(body.link),
    })
}

pub async fn update_carousel(db: &DB, id: i32, body: UpdateCarouselBody) -> Result<CarouselItem, AppError> {
    let existing = sqlx::query(
        "SELECT id, title, subtitle, image_dataurl
         FROM carousel_items
         WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&db.pool)
    .await?;

    let Some(old) = existing else {
        return Err(AppError::not_found("CAROUSEL_NOT_FOUND", "Carousel item not found"));
    };

    let image_dataurl: String = body.image_url.unwrap_or_else(|| old.get("image_dataurl"));
    let title: Option<String> = body.title.or(old.try_get("title").ok());
    let subtitle: Option<String> = body.subtitle.or(old.try_get("subtitle").ok());

    let _ = sqlx::query(
        "UPDATE carousel_items
         SET image_dataurl = $2,
             title = $3,
             subtitle = $4,
             updated_at = NOW()
         WHERE id = $1",
    )
    .bind(id)
    .bind(&image_dataurl)
    .bind(&title)
    .bind(&subtitle)
    .execute(&db.pool)
    .await?;

    Ok(CarouselItem {
        id,
        image_url: image_dataurl,
        title,
        subtitle,
        link: body.link,
    })
}

pub async fn delete_carousel(db: &DB, id: i32) -> Result<(), AppError> {
    let res = sqlx::query("DELETE FROM carousel_items WHERE id = $1")
        .bind(id)
        .execute(&db.pool)
        .await?;

    if res.rows_affected() == 0 {
        return Err(AppError::not_found("CAROUSEL_NOT_FOUND", "Carousel item not found"));
    }
    Ok(())
}

// -------------------- Debug helpers (keep) --------------------

pub async fn get_verification_token(_db: &DB, _email: String) -> Result<String, AppError> {
    Ok("".to_string())
}

pub async fn get_reset_token(_db: &DB, _email: String) -> Result<String, AppError> {
    Ok("".to_string())
}
