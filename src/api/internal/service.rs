use sqlx::Row;
use crate::config::db::DB;
use crate::core::errors::AppError;
use super::schema::*;

// --- User Management ---

pub async fn find_user(db: &DB, body: FindUserBody) -> Result<UserLite, AppError> {
    let email = body.email.trim().to_lowercase();
    if email.is_empty() {
        return Err(AppError::bad_request("email is required"));
    }

    let row = sqlx::query(
        r#"
        SELECT id, email, username, role, provider, is_verified
        FROM users
        WHERE LOWER(email) = $1
        "#,
    )
    .bind(&email)
    .fetch_optional(&db.pool)
    .await?;

    let Some(row) = row else {
        return Err(AppError::not_found("USER_NOT_FOUND", "User not found"));
    };

    Ok(UserLite {
        id: row.get("id"),
        email: row.get("email"),
        username: row.get("username"),
        role: row.get("role"),
        provider: row.get("provider"),
        is_verified: row.get("is_verified"),
    })
}

pub async fn create_user_email(db: &DB, body: CreateUserEmailBody) -> Result<UserLite, AppError> {
    let email = body.email.trim().to_lowercase();
    if email.is_empty() {
        return Err(AppError::bad_request("email is required"));
    }
    
    let default_username = email.split('@').next().unwrap_or("user").to_string();

    let row = sqlx::query(
        r#"
        INSERT INTO users (email, username, role, is_verified, provider)
        VALUES ($1, $2, 'user', FALSE, 'local')
        RETURNING id, email, username, role, provider, is_verified
        "#,
    )
    .bind(&email)
    .bind(&default_username)
    .fetch_one(&db.pool)
    .await
    .map_err(|e| AppError::internal(format!("DB Error: {}", e)))?;

    Ok(UserLite {
        id: row.get("id"),
        email: row.get("email"),
        username: row.get("username"),
        role: row.get("role"),
        provider: row.get("provider"),
        is_verified: row.get("is_verified"),
    })
}

pub async fn get_verification_token(db: &DB, email: String) -> Result<String, AppError> {
    let email = email.trim().to_lowercase();
    if email.is_empty() { return Err(AppError::bad_request("email is required")); }

    let row = sqlx::query(
        "SELECT code FROM verification_codes WHERE user_id = (SELECT id FROM users WHERE email=$1) ORDER BY id DESC LIMIT 1"
    )
    .bind(&email)
    .fetch_optional(&db.pool)
    .await?;
    
    let Some(row) = row else { return Err(AppError::not_found("TOKEN_NOT_FOUND", "Token not found")); };
    Ok(row.get("code"))
}

pub async fn get_reset_token(db: &DB, email: String) -> Result<String, AppError> {
    let email = email.trim().to_lowercase();
    if email.is_empty() { return Err(AppError::bad_request("email is required")); }

    let row = sqlx::query(
        "SELECT token FROM password_reset_tokens WHERE user_id = (SELECT id FROM users WHERE email=$1) ORDER BY id DESC LIMIT 1"
    )
    .bind(&email)
    .fetch_optional(&db.pool)
    .await?;

    let Some(row) = row else { return Err(AppError::not_found("TOKEN_NOT_FOUND", "Token not found")); };
    Ok(row.get("token"))
}

pub async fn list_users(db: &DB) -> Result<Vec<UserLite>, AppError> {
    let rows = sqlx::query(
        r#"
        SELECT id, email, username, role, provider, is_verified
        FROM users
        ORDER BY id DESC
        "#,
    )
    .fetch_all(&db.pool)
    .await?;

    let mut out = Vec::with_capacity(rows.len());
    for r in rows {
        out.push(UserLite {
            id: r.get("id"),
            email: r.get("email"),
            username: r.get("username"),
            role: r.get("role"),
            provider: r.get("provider"),
            is_verified: r.get("is_verified"),
        });
    }
    Ok(out)
}

// --- Client Management ---

pub async fn list_clients(db: &DB) -> Result<Vec<ClientRow>, AppError> {
    let rows = sqlx::query("SELECT id, name, api_key, is_active FROM api_clients ORDER BY id DESC")
        .fetch_all(&db.pool).await?;
    let mut out = Vec::new();
    for r in rows { out.push(ClientRow { id: r.get("id"), name: r.get("name"), api_key: r.get("api_key"), is_active: r.get("is_active") }); }
    Ok(out)
}

pub async fn set_client_active(db: &DB, id: i32, is_active: bool) -> Result<(), AppError> {
    let res = sqlx::query("UPDATE api_clients SET is_active = $2 WHERE id = $1").bind(id).bind(is_active).execute(&db.pool).await?;
    if res.rows_affected() == 0 { return Err(AppError::not_found("CLIENT_NOT_FOUND", "Client not found")); }
    Ok(())
}

// --- Content Management (Homepage & Carousel) ---

pub async fn get_homepage_hero(db: &DB) -> Result<HomepageHero, AppError> {
    let row = sqlx::query("SELECT title, subtitle, cta_text, cta_link FROM homepage_hero ORDER BY id DESC LIMIT 1").fetch_optional(&db.pool).await?;
    if let Some(r) = row {
        return Ok(HomepageHero {
            title: r.get("title"),
            subtitle: r.get("subtitle"),
            cta_text: r.get("cta_text"),
            cta_link: r.get("cta_link"),
        });
    }
    // Fallback default
    Ok(HomepageHero {
        title: "Welcome".into(),
        subtitle: Some("Pure API running".into()),
        cta_text: Some("Get Started".into()),
        cta_link: Some("/".into()),
    })
}

pub async fn put_homepage_hero(db: &DB, body: HomepageHeroBody) -> Result<HomepageHero, AppError> {
    if body.title.trim().is_empty() { return Err(AppError::bad_request("title is required")); }

    let row = sqlx::query(
        r#"
        INSERT INTO homepage_hero (title, subtitle, cta_text, cta_link)
        VALUES ($1, $2, $3, $4)
        RETURNING title, subtitle, cta_text, cta_link
        "#
    )
    .bind(body.title.trim())
    .bind(body.subtitle.trim())
    .bind(body.cta_text.trim())
    .bind(body.cta_link.trim())
    .fetch_one(&db.pool).await?;

    Ok(HomepageHero {
        title: row.get("title"),
        subtitle: row.get("subtitle"),
        cta_text: row.get("cta_text"),
        cta_link: row.get("cta_link"),
    })
}

pub async fn get_carousel(db: &DB) -> Result<Vec<CarouselItem>, AppError> {
    let rows = sqlx::query(
        "SELECT id, image_url, title, subtitle, link FROM carousel_items ORDER BY id DESC"
    )
    .fetch_all(&db.pool).await?;

    let mut out = Vec::new();
    for r in rows {
        out.push(CarouselItem {
            id: r.get("id"),
            image_url: r.get("image_url"),
            title: r.get("title"),
            subtitle: r.get("subtitle"),
            link: r.get("link"),
        });
    }
    Ok(out)
}

pub async fn create_carousel(db: &DB, body: CreateCarouselBody) -> Result<CarouselItem, AppError> {
    if body.image_url.trim().is_empty() { return Err(AppError::bad_request("image_url is required")); }
    if body.title.trim().is_empty() { return Err(AppError::bad_request("title is required")); }

    let row = sqlx::query(
        r#"
        INSERT INTO carousel_items (image_url, title, subtitle, link)
        VALUES ($1, $2, $3, $4)
        RETURNING id, image_url, title, subtitle, link
        "#
    )
    .bind(body.image_url.trim())
    .bind(body.title.trim())
    .bind(body.subtitle.trim())
    .bind(body.link.trim())
    .fetch_one(&db.pool).await?;

    Ok(CarouselItem {
        id: row.get("id"),
        image_url: row.get("image_url"),
        title: row.get("title"),
        subtitle: row.get("subtitle"),
        link: row.get("link"),
    })
}

pub async fn update_carousel(db: &DB, id: i32, body: UpdateCarouselBody) -> Result<CarouselItem, AppError> {
    let existing = sqlx::query("SELECT id, image_url, title, subtitle, link FROM carousel_items WHERE id = $1")
        .bind(id)
        .fetch_optional(&db.pool).await?;

    let Some(existing) = existing else {
        return Err(AppError::not_found("CAROUSEL_NOT_FOUND", "Carousel item not found"));
    };

    let mut image_url: String = existing.get("image_url");
    let mut title: String = existing.try_get("title").unwrap_or_default();
    let mut subtitle: String = existing.try_get("subtitle").unwrap_or_default();
    let mut link: String = existing.try_get("link").unwrap_or_default();

    if let Some(v) = body.image_url { if !v.trim().is_empty() { image_url = v.trim().to_string(); } }
    if let Some(v) = body.title { if !v.trim().is_empty() { title = v.trim().to_string(); } }
    if let Some(v) = body.subtitle { subtitle = v.trim().to_string(); }
    if let Some(v) = body.link { link = v.trim().to_string(); }

    let row = sqlx::query(
        r#"
        UPDATE carousel_items
        SET image_url=$2, title=$3, subtitle=$4, link=$5
        WHERE id=$1
        RETURNING id, image_url, title, subtitle, link
        "#
    )
    .bind(id)
    .bind(&image_url)
    .bind(&title)
    .bind(&subtitle)
    .bind(&link)
    .fetch_one(&db.pool).await?;

    Ok(CarouselItem {
        id: row.get("id"),
        image_url: row.get("image_url"),
        title: row.get("title"),
        subtitle: row.get("subtitle"),
        link: row.get("link"),
    })
}

pub async fn delete_carousel(db: &DB, id: i32) -> Result<(), AppError> {
    let res = sqlx::query("DELETE FROM carousel_items WHERE id = $1")
        .bind(id)
        .execute(&db.pool).await?;

    if res.rows_affected() == 0 {
        return Err(AppError::not_found("CAROUSEL_NOT_FOUND", "Carousel item not found"));
    }
    Ok(())
}