use sqlx::Row;
use crate::config::db::DB;
use crate::core::errors::AppError;
use super::schema::*;
use bcrypt::{hash, DEFAULT_COST};
use chrono::{DateTime, Utc};

// มาโครสำหรับช่วย Map Row ให้เป็น UserLite เพื่อลดความซ้ำซ้อนและกัน Error
macro_rules! map_user_lite {
    ($r:expr) => {
        UserLite {
            id: $r.get("id"),
            email: $r.get("email"),
            username: $r.try_get("username").unwrap_or(None),
            first_name: $r.try_get("first_name").unwrap_or(None),
            last_name: $r.try_get("last_name").unwrap_or(None),
            tel: $r.try_get("tel").unwrap_or(None),
            status: $r.try_get("status").unwrap_or(None),
            role: $r.try_get("role").unwrap_or_else(|_| "user".to_string()),
            password_hash: $r.try_get("password_hash").unwrap_or(None),
            is_email_verified: $r.try_get("is_email_verified").unwrap_or(false),
            oauth_provider: $r.try_get("oauth_provider").unwrap_or(None),
            profile_picture_url: $r.try_get("profile_picture_url").unwrap_or(None),
        }
    };
}

// --- User Management ---

pub async fn find_user(db: &DB, body: FindUserBody) -> Result<UserLite, AppError> {
    let query_str = "SELECT id::text AS id, email, username, first_name, last_name, tel, status, role, password_hash, oauth_provider, is_email_verified, profile_picture_url FROM users";
    
    let row = if let Some(email) = &body.email {
        sqlx::query(&format!("{} WHERE LOWER(email) = $1", query_str))
            .bind(email.trim().to_lowercase())
            .fetch_optional(&db.pool)
            .await?
    } else if let Some(id) = body.id {
        sqlx::query(&format!("{} WHERE id = $1::uuid", query_str))
            .bind(id)
            .fetch_optional(&db.pool)
            .await?
    } else if let (Some(p), Some(oid)) = (&body.provider, &body.oauth_id) {
        sqlx::query(&format!("{} WHERE oauth_provider = $1 AND oauth_id = $2", query_str))
            .bind(p)
            .bind(oid)
            .fetch_optional(&db.pool)
            .await?
    } else {
        return Err(AppError::bad_request("Missing search criteria"));
    };

    let r = row.ok_or_else(|| AppError::not_found("USER_NOT_FOUND", "User not found"))?;
    Ok(map_user_lite!(r))
}

pub async fn create_user_email(db: &DB, body: CreateUserEmailBody) -> Result<UserLite, AppError> {
    let email = body.email.trim().to_lowercase();
    let default_username = email.split('@').next().unwrap_or("user").to_string();
    let new_id = uuid::Uuid::now_v7().to_string();

    let r = sqlx::query(
        "INSERT INTO users (id, email, username, role, status, is_email_verified, oauth_provider) 
         VALUES ($1::uuid, $2, $3, 'user', 'active', FALSE, 'local') 
         RETURNING id::text AS id, email, username, first_name, last_name, tel, status, role, password_hash, oauth_provider, is_email_verified, profile_picture_url"
    )
    .bind(&new_id)
    .bind(&email)
    .bind(&default_username)
    .fetch_one(&db.pool)
    .await
    .map_err(|e| AppError::internal(format!("DB Error: {}", e)))?;

    Ok(map_user_lite!(r))
}

pub async fn set_oauth_user(db: &DB, body: SetOAuthUserBody) -> Result<UserLite, AppError> {
    let email = body.email.trim().to_lowercase();
    let existing = sqlx::query("SELECT id::text AS id FROM users WHERE LOWER(email) = $1")
        .bind(&email)
        .fetch_optional(&db.pool)
        .await?;

    let r = if let Some(u) = existing {
        let user_id: String = u.get("id");
        sqlx::query(
            "UPDATE users 
             SET oauth_provider = $2, oauth_id = $3, is_email_verified = TRUE, profile_picture_url = COALESCE($4, profile_picture_url), username = COALESCE(username, $5) 
             WHERE id = $1::uuid 
             RETURNING id::text AS id, email, username, first_name, last_name, tel, status, role, password_hash, oauth_provider, is_email_verified, profile_picture_url"
        )
        .bind(&user_id)
        .bind(&body.provider)
        .bind(&body.oauth_id)
        .bind(&body.picture_url)
        .bind(&body.name)
        .fetch_one(&db.pool)
        .await?
    } else {
        let username = body.name.clone().unwrap_or_else(|| email.split('@').next().unwrap_or("user").to_string());
        let new_id = uuid::Uuid::now_v7().to_string();
        sqlx::query(
            "INSERT INTO users (id, email, username, role, status, is_email_verified, oauth_provider, oauth_id, profile_picture_url) 
             VALUES ($1::uuid, $2, $3, 'user', 'active', TRUE, $4, $5, $6) 
             RETURNING id::text AS id, email, username, first_name, last_name, tel, status, role, password_hash, oauth_provider, is_email_verified, profile_picture_url"
        )
        .bind(&new_id)
        .bind(&email)
        .bind(&username)
        .bind(&body.provider)
        .bind(&body.oauth_id)
        .bind(&body.picture_url)
        .fetch_one(&db.pool)
        .await?
    };

    Ok(map_user_lite!(r))
}

pub async fn set_username_password(db: &DB, body: SetUsernamePasswordBody) -> Result<UserLite, AppError> {
    let email = body.email.trim().to_lowercase();
    let hash = hash(body.password, DEFAULT_COST).map_err(|_| AppError::internal("Hash error"))?;

    let r = sqlx::query(
        "UPDATE users 
         SET username = COALESCE($2, username), 
             password_hash = $3, 
             first_name = COALESCE($4, first_name), 
             last_name = COALESCE($5, last_name), 
             tel = COALESCE($6, tel) 
         WHERE LOWER(email) = $1 
         RETURNING id::text AS id, email, username, first_name, last_name, tel, status, role, password_hash, oauth_provider, is_email_verified, profile_picture_url"
    )
    .bind(&email)
    .bind(&body.username)
    .bind(hash)
    .bind(&body.first_name)
    .bind(&body.last_name)
    .bind(&body.tel)
    .fetch_optional(&db.pool)
    .await?;

    let Some(r) = r else { return Err(AppError::not_found("USER_NOT_FOUND", "User not found")); };
    Ok(map_user_lite!(r))
}

pub async fn update_user(db: &DB, body: UpdateUserBody) -> Result<UserLite, AppError> {
    let r = sqlx::query(
        "UPDATE users 
         SET username = COALESCE($2, username), 
             profile_picture_url = COALESCE($3, profile_picture_url),
             first_name = COALESCE($4, first_name),
             last_name = COALESCE($5, last_name),
             tel = COALESCE($6, tel),
             status = COALESCE($7, status),
             role = COALESCE($8, role),
             updated_at = NOW()
         WHERE id = $1::uuid 
         RETURNING id::text AS id, email, username, first_name, last_name, tel, status, role, password_hash, oauth_provider, is_email_verified, profile_picture_url"
    )
    .bind(&body.id)
    .bind(&body.username)
    .bind(&body.profile_picture_url)
    .bind(&body.first_name)
    .bind(&body.last_name)
    .bind(&body.tel)
    .bind(&body.status)
    .bind(&body.role)
    .fetch_optional(&db.pool)
    .await?;

    let Some(r) = r else {
        return Err(AppError::not_found("USER_NOT_FOUND", "User not found"));
    };

    Ok(map_user_lite!(r))
}

pub async fn delete_user(db: &DB, body: DeleteUserBody) -> Result<(), AppError> {
    let res = sqlx::query("DELETE FROM users WHERE id = $1::uuid")
        .bind(&body.id)
        .execute(&db.pool)
        .await?;

    if res.rows_affected() == 0 {
        return Err(AppError::not_found("USER_NOT_FOUND", "User not found"));
    }
    Ok(())
}

pub async fn store_verification_code(db: &DB, body: StoreVerificationCodeBody) -> Result<(), AppError> {
    let expires_at = DateTime::parse_from_rfc3339(&body.expires_at)
        .map_err(|_| AppError::bad_request("Invalid date format"))?
        .with_timezone(&Utc);

    sqlx::query("DELETE FROM verification_codes WHERE user_id = $1::uuid")
        .bind(&body.user_id)
        .execute(&db.pool)
        .await?;

    sqlx::query("INSERT INTO verification_codes (id, user_id, code, expires_at) VALUES ($1::uuid, $2::uuid, $3, $4)")
        .bind(uuid::Uuid::now_v7().to_string())
        .bind(&body.user_id)
        .bind(&body.code)
        .bind(expires_at)
        .execute(&db.pool)
        .await?;

    Ok(())
}

pub async fn verify_code(db: &DB, body: VerifyCodeBody) -> Result<VerifyCodeResponse, AppError> {
    let email = body.email.trim().to_lowercase();
    let user = sqlx::query("SELECT id::text AS id FROM users WHERE LOWER(email) = $1")
        .bind(&email)
        .fetch_optional(&db.pool)
        .await?;

    let Some(user) = user else {
        return Err(AppError::not_found("USER_NOT_FOUND", "User not found"));
    };

    let user_id: String = user.get("id");

    let code_row = sqlx::query("SELECT id::text AS id FROM verification_codes WHERE user_id = $1::uuid AND code = $2 AND expires_at > NOW() LIMIT 1")
        .bind(&user_id)
        .bind(&body.code)
        .fetch_optional(&db.pool)
        .await?;

    if let Some(c) = code_row {
        let code_id: String = c.get("id");
        let mut tx = db.pool.begin().await?;

        sqlx::query("DELETE FROM verification_codes WHERE id = $1::uuid").bind(&code_id).execute(&mut *tx).await?;
        sqlx::query("UPDATE users SET is_email_verified = TRUE WHERE id = $1::uuid").bind(&user_id).execute(&mut *tx).await?;

        tx.commit().await?;
        Ok(VerifyCodeResponse { ok: true, user_id: Some(user_id), reason: None })
    } else {
        Ok(VerifyCodeResponse { ok: false, user_id: None, reason: Some("Invalid or expired code".into()) })
    }
}

pub async fn create_reset_token(db: &DB, body: CreateResetTokenBody) -> Result<(), AppError> {
    let user = sqlx::query("SELECT id::text AS id FROM users WHERE LOWER(email) = $1")
        .bind(&body.email)
        .fetch_optional(&db.pool)
        .await?;

    if let Some(u) = user {
        let user_id: String = u.get("id");
        let expires_at = DateTime::parse_from_rfc3339(&body.expires_at)
            .map_err(|_| AppError::bad_request("Invalid date"))?
            .with_timezone(&Utc);

        sqlx::query("DELETE FROM password_reset_tokens WHERE user_id = $1::uuid").bind(&user_id).execute(&db.pool).await?;
        sqlx::query("INSERT INTO password_reset_tokens (id, user_id, token, expires_at) VALUES ($1::uuid, $2::uuid, $3, $4)")
            .bind(uuid::Uuid::now_v7().to_string()).bind(&user_id).bind(&body.token).bind(expires_at).execute(&db.pool).await?;
    }
    Ok(())
}

pub async fn consume_reset_token(db: &DB, body: ConsumeResetTokenBody) -> Result<UserLite, AppError> {
    let row = sqlx::query("SELECT user_id::text AS user_id FROM password_reset_tokens WHERE token = $1 AND expires_at > NOW() AND is_used = FALSE")
        .bind(&body.token)
        .fetch_optional(&db.pool)
        .await?;

    if let Some(r) = row {
        let user_id: String = r.get("user_id");

        sqlx::query("UPDATE password_reset_tokens SET is_used = TRUE WHERE token = $1")
            .bind(&body.token)
            .execute(&db.pool)
            .await?;

        let r = sqlx::query(
            "SELECT id::text AS id, email, username, first_name, last_name, tel, status, role, password_hash, oauth_provider, is_email_verified, profile_picture_url FROM users WHERE id = $1::uuid"
        )
        .bind(&user_id)
        .fetch_one(&db.pool)
        .await?;

        Ok(map_user_lite!(r))
    } else {
        Err(AppError::bad_request("Invalid or expired token"))
    }
}

pub async fn set_password(db: &DB, body: SetPasswordBody) -> Result<(), AppError> {
    let hash = hash(body.new_password, DEFAULT_COST).map_err(|_| AppError::internal("Hash error"))?;
    sqlx::query("UPDATE users SET password_hash = $2 WHERE id = $1::uuid")
        .bind(&body.user_id)
        .bind(hash)
        .execute(&db.pool)
        .await?;
    Ok(())
}

pub async fn list_users(db: &DB) -> Result<Vec<UserLite>, AppError> {
    let rows = sqlx::query(
        "SELECT id::text AS id, email, username, first_name, last_name, tel, status, role, password_hash, oauth_provider, is_email_verified, profile_picture_url FROM users ORDER BY created_at DESC"
    )
    .fetch_all(&db.pool)
    .await?;

    let mut out = Vec::new();
    for r in rows {
        out.push(map_user_lite!(r));
    }
    Ok(out)
}

pub async fn list_clients(db: &DB) -> Result<Vec<ClientRow>, AppError> {
    let rows = sqlx::query("SELECT id::text AS id, name, api_key, is_active FROM api_clients ORDER BY created_at DESC")
        .fetch_all(&db.pool)
        .await?;

    let mut out = Vec::new();
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

pub async fn set_client_active(db: &DB, id: &str, is_active: bool) -> Result<(), AppError> {
    let res = sqlx::query("UPDATE api_clients SET is_active = $2 WHERE id = $1::uuid")
        .bind(id)
        .bind(is_active)
        .execute(&db.pool)
        .await?;

    if res.rows_affected() == 0 {
        return Err(AppError::not_found("CLIENT_NOT_FOUND", "Client not found"));
    }
    Ok(())
}

// --- Homepage ---

pub async fn get_homepage_content(db: &DB) -> Result<Vec<HomepageContentRow>, AppError> {
    let rows = sqlx::query("SELECT section_name, content FROM homepage_content")
        .fetch_all(&db.pool)
        .await?;

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
    sqlx::query(
        "INSERT INTO homepage_content (section_name, content, updated_at) VALUES ($1, $2, NOW())
         ON CONFLICT (section_name) DO UPDATE SET content = $2, updated_at = NOW()"
    )
    .bind(&body.section_name)
    .bind(&body.content)
    .execute(&db.pool)
    .await?;

    Ok(HomepageContentRow {
        section_name: body.section_name,
        content: body.content,
    })
}

// --- Carousel ---

pub async fn get_carousel(db: &DB) -> Result<Vec<CarouselItem>, AppError> {
    let rows = sqlx::query(
        "SELECT id::text AS id, item_index, title, subtitle, description, image_dataurl
         FROM carousel_items
         ORDER BY item_index ASC, created_at DESC"
    )
    .fetch_all(&db.pool)
    .await?;

    let mut out = Vec::new();
    for r in rows {
        out.push(CarouselItem {
            id: r.get("id"),
            item_index: r.get("item_index"),
            image_dataurl: r.get("image_dataurl"),
            title: r.try_get("title").ok(),
            subtitle: r.try_get("subtitle").ok(),
            description: r.try_get("description").ok(),
        });
    }
    Ok(out)
}

pub async fn create_carousel(db: &DB, body: CreateCarouselBody) -> Result<CarouselItem, AppError> {
    let new_id = uuid::Uuid::now_v7().to_string();
    let row = sqlx::query(
        "INSERT INTO carousel_items (id, item_index, image_dataurl, title, subtitle, description)
         VALUES ($1::uuid, $2, $3, $4, $5, $6) RETURNING id::text AS id"
    )
    .bind(&new_id)
    .bind(body.item_index.unwrap_or(0))
    .bind(&body.image_url)
    .bind(&body.title)
    .bind(&body.subtitle)
    .bind(&body.description)
    .fetch_one(&db.pool)
    .await?;

    Ok(CarouselItem {
        id: row.get("id"),
        item_index: body.item_index.unwrap_or(0),
        image_dataurl: body.image_url,
        title: body.title,
        subtitle: body.subtitle,
        description: body.description,
    })
}

pub async fn update_carousel(db: &DB, body: UpdateCarouselBody) -> Result<CarouselItem, AppError> {
    let id = &body.id;
    let existing = sqlx::query("SELECT id::text AS id, item_index, title, subtitle, description, image_dataurl FROM carousel_items WHERE id = $1::uuid")
        .bind(id)
        .fetch_optional(&db.pool)
        .await?;

    let Some(existing) = existing else {
        return Err(AppError::not_found("CAROUSEL_NOT_FOUND", "Not found"));
    };

    let new_item_index: i32 = body.item_index.unwrap_or(existing.get("item_index"));
    let new_image = body.image_url.unwrap_or(existing.get("image_dataurl"));
    let new_title = body.title.or(existing.try_get("title").ok());
    let new_subtitle = body.subtitle.or(existing.try_get("subtitle").ok());
    let new_desc = body.description.or(existing.try_get("description").ok());

    let row = sqlx::query(
        "UPDATE carousel_items
         SET item_index=$2, image_dataurl=$3, title=$4, subtitle=$5, description=$6, updated_at=NOW()
         WHERE id=$1::uuid RETURNING id::text AS id"
    )
    .bind(id)
    .bind(new_item_index)
    .bind(&new_image)
    .bind(&new_title)
    .bind(&new_subtitle)
    .bind(&new_desc)
    .fetch_one(&db.pool)
    .await?;

    Ok(CarouselItem {
        id: row.get("id"),
        item_index: new_item_index,
        image_dataurl: new_image,
        title: new_title,
        subtitle: new_subtitle,
        description: new_desc,
    })
}

pub async fn delete_carousel(db: &DB, body: DeleteCarouselBody) -> Result<(), AppError> {
    sqlx::query("DELETE FROM carousel_items WHERE id = $1::uuid")
        .bind(&body.id)
        .execute(&db.pool)
        .await?;
    Ok(())
}

pub async fn get_verification_token(_db: &DB, _email: String) -> Result<String, AppError> { Ok("".into()) }
pub async fn get_reset_token(_db: &DB, _email: String) -> Result<String, AppError> { Ok("".into()) }