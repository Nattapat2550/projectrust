use sqlx::Row;
use crate::config::db::DB;
use crate::core::errors::AppError;
use super::schema::*;

// --- User Management ---

pub async fn find_user(db: &DB, body: FindUserBody) -> Result<UserLite, AppError> {
    let email = body.email.trim().to_lowercase();
    if email.is_empty() { return Err(AppError::bad_request("email is required")); }

    let row = sqlx::query(
        "SELECT id, email, username, role, provider, is_verified FROM users WHERE LOWER(email) = $1"
    )
    .bind(&email).fetch_optional(&db.pool).await?;

    let Some(row) = row else { return Err(AppError::not_found("USER_NOT_FOUND", "User not found")); };

    Ok(UserLite {
        id: row.get("id"), email: row.get("email"), username: row.get("username"),
        role: row.get("role"), provider: row.get("provider"), is_verified: row.get("is_verified"),
    })
}

pub async fn create_user_email(db: &DB, body: CreateUserEmailBody) -> Result<UserLite, AppError> {
    let email = body.email.trim().to_lowercase();
    let default_username = email.split('@').next().unwrap_or("user").to_string();

    let row = sqlx::query(
        r#"
        INSERT INTO users (email, username, role, is_verified, provider)
        VALUES ($1, $2, 'user', FALSE, 'local')
        RETURNING id, email, username, role, provider, is_verified
        "#
    )
    .bind(&email).bind(&default_username).fetch_one(&db.pool).await
    .map_err(|e| AppError::internal(format!("DB Error: {}", e)))?;

    Ok(UserLite {
        id: row.get("id"), email: row.get("email"), username: row.get("username"),
        role: row.get("role"), provider: row.get("provider"), is_verified: row.get("is_verified"),
    })
}

pub async fn list_users(db: &DB) -> Result<Vec<UserLite>, AppError> {
    let rows = sqlx::query("SELECT id, email, username, role, provider, is_verified FROM users ORDER BY id DESC")
        .fetch_all(&db.pool).await?;
    let mut out = Vec::new();
    for r in rows {
        out.push(UserLite {
            id: r.get("id"), email: r.get("email"), username: r.get("username"),
            role: r.get("role"), provider: r.get("provider"), is_verified: r.get("is_verified"),
        });
    }
    Ok(out)
}

pub async fn get_verification_token(db: &DB, email: String) -> Result<String, AppError> {
     let row = sqlx::query("SELECT code FROM verification_codes WHERE user_id=(SELECT id FROM users WHERE email=$1) ORDER BY id DESC LIMIT 1").bind(email).fetch_optional(&db.pool).await?;
     Ok(row.map(|r| r.get("code")).unwrap_or_default())
}

pub async fn get_reset_token(db: &DB, email: String) -> Result<String, AppError> {
     let row = sqlx::query("SELECT token FROM password_reset_tokens WHERE user_id=(SELECT id FROM users WHERE email=$1) ORDER BY id DESC LIMIT 1").bind(email).fetch_optional(&db.pool).await?;
     Ok(row.map(|r| r.get("token")).unwrap_or_default())
}

// --- Client Management ---

pub async fn list_clients(db: &DB) -> Result<Vec<ClientRow>, AppError> {
    let rows = sqlx::query(
        "SELECT id, name, api_key, is_active FROM api_clients ORDER BY id DESC"
    )
    .fetch_all(&db.pool).await?;

    let mut out = Vec::new();
    for r in rows {
        out.push(ClientRow {
            id: r.get("id"), name: r.get("name"), api_key: r.get("api_key"), is_active: r.get("is_active")
        });
    }
    Ok(out)
}

pub async fn set_client_active(db: &DB, id: i32, is_active: bool) -> Result<(), AppError> {
    let res = sqlx::query("UPDATE api_clients SET is_active = $2 WHERE id = $1")
        .bind(id).bind(is_active).execute(&db.pool).await?;
    
    if res.rows_affected() == 0 { return Err(AppError::not_found("CLIENT_NOT_FOUND", "Client not found")); }
    Ok(())
}

// --- Homepage & Carousel ---

pub async fn get_homepage_hero(db: &DB) -> Result<HomepageHero, AppError> {
    let row = sqlx::query("SELECT title, subtitle, cta_text, cta_link FROM homepage_content WHERE section_name='hero'")
        .fetch_optional(&db.pool).await?;
    
    if let Some(r) = row {
         // Map 'content' to 'subtitle' for compatibility as database stores content block
         return Ok(HomepageHero { 
             title: "Welcome".into(), 
             subtitle: Some(r.get("content")), 
             cta_text: None, 
             cta_link: None 
         });
    }
    Ok(HomepageHero{title:"Welcome".into(), subtitle:Some("Pure API".into()), cta_text:None, cta_link:None})
}

pub async fn put_homepage_hero(db: &DB, body: HomepageHeroBody) -> Result<HomepageHero, AppError> {
    sqlx::query(
        "INSERT INTO homepage_content (section_name, content) VALUES ('hero', $1) 
         ON CONFLICT (section_name) DO UPDATE SET content = $1"
    )
    .bind(&body.subtitle)
    .execute(&db.pool).await?;

    Ok(HomepageHero{title:body.title, subtitle:Some(body.subtitle), cta_text:Some(body.cta_text), cta_link:Some(body.cta_link)})
}

pub async fn get_carousel(db: &DB) -> Result<Vec<CarouselItem>, AppError> {
    let rows = sqlx::query(
        "SELECT id, title, subtitle, image_dataurl FROM carousel_items ORDER BY id DESC"
    )
    .fetch_all(&db.pool).await?;

    let mut out = Vec::new();
    for r in rows {
        out.push(CarouselItem{
            id:r.get("id"), 
            image_url:r.get("image_dataurl"), 
            title:r.try_get("title").ok(), 
            subtitle:r.try_get("subtitle").ok(), 
            link:None
        }); 
    }
    Ok(out)
}

pub async fn create_carousel(db: &DB, body: CreateCarouselBody) -> Result<CarouselItem, AppError> {
    let row = sqlx::query(
        "INSERT INTO carousel_items (image_dataurl, title, subtitle) VALUES ($1, $2, $3) RETURNING id"
    )
    .bind(&body.image_url).bind(&body.title).bind(&body.subtitle)
    .fetch_one(&db.pool).await?;

    Ok(CarouselItem{
        id:row.get("id"), 
        image_url:body.image_url, 
        title:Some(body.title), 
        subtitle:Some(body.subtitle), 
        link:Some(body.link)
    })
}

// ✅ แก้ไข: Implement logic จริง และใช้ db เพื่อแก้ Warning
pub async fn update_carousel(db: &DB, id: i32, body: UpdateCarouselBody) -> Result<CarouselItem, AppError> {
    // 1. ดึงข้อมูลเก่า
    let existing = sqlx::query("SELECT id, title, subtitle, image_dataurl FROM carousel_items WHERE id = $1")
        .bind(id)
        .fetch_optional(&db.pool).await?;

    let Some(existing) = existing else {
        return Err(AppError::not_found("CAROUSEL_NOT_FOUND", "Carousel item not found"));
    };

    // 2. เตรียมข้อมูลใหม่ (ถ้า body ส่งมาเป็น None ให้ใช้ค่าเดิม)
    let old_image: String = existing.get("image_dataurl");
    let old_title: Option<String> = existing.try_get("title").ok();
    let old_subtitle: Option<String> = existing.try_get("subtitle").ok();

    let new_image = body.image_url.unwrap_or(old_image);
    let new_title = body.title.or(old_title);
    let new_subtitle = body.subtitle.or(old_subtitle);

    // 3. Update
    let row = sqlx::query(
        r#"
        UPDATE carousel_items 
        SET image_dataurl = $2, title = $3, subtitle = $4
        WHERE id = $1
        RETURNING id, title, subtitle, image_dataurl
        "#
    )
    .bind(id)
    .bind(new_image)
    .bind(new_title)
    .bind(new_subtitle)
    .fetch_one(&db.pool).await?;

    Ok(CarouselItem {
        id: row.get("id"),
        image_url: row.get("image_dataurl"),
        title: row.try_get("title").ok(),
        subtitle: row.try_get("subtitle").ok(),
        link: body.link, // DB ไม่มี column link แต่คืนค่ากลับไปเพื่อให้ Frontend ไม่ error
    })
}

pub async fn delete_carousel(db: &DB, id: i32) -> Result<(), AppError> { 
    sqlx::query("DELETE FROM carousel_items WHERE id = $1").bind(id).execute(&db.pool).await?;
    Ok(()) 
}