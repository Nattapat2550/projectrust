use sqlx::Row;
use serde_json::json;
use crate::config::db::DB;
use crate::core::errors::AppError;
use super::schema::{HomepageHero, HomepageHeroBody};

// ✅ Helper constant สำหรับ Default Hero
fn default_hero() -> HomepageHero {
    HomepageHero {
        title: "Welcome".into(),
        subtitle: "Pure API running".into(),
        cta_text: "Get Started".into(),
        cta_link: "/".into(),
    }
}

pub async fn get_hero(db: &DB) -> Result<HomepageHero, AppError> {
    // ✅ แก้ SQL: ดึง content จาก homepage_content where section_name = 'hero'
    let row = sqlx::query(
        "SELECT content FROM homepage_content WHERE section_name = 'hero'"
    )
    .fetch_optional(&db.pool)
    .await?;

    if let Some(r) = row {
        let content_str: String = r.get("content");
        // Parse JSON String -> Struct
        let hero: HomepageHero = serde_json::from_str(&content_str).unwrap_or(default_hero());
        return Ok(hero);
    }

    Ok(default_hero())
}

pub async fn put_hero(db: &DB, body: HomepageHeroBody) -> Result<HomepageHero, AppError> {
    if body.title.trim().is_empty() {
        return Err(AppError::bad_request("title is required"));
    }

    // Convert Struct -> JSON String
    let content_json = json!({
        "title": body.title.trim(),
        "subtitle": body.subtitle.trim(),
        "cta_text": body.cta_text.trim(),
        "cta_link": body.cta_link.trim(),
    }).to_string();

    // ✅ แก้ SQL: Upsert ลง homepage_content
    sqlx::query(
        r#"
        INSERT INTO homepage_content (section_name, content, updated_at)
        VALUES ('hero', $1, NOW())
        ON CONFLICT (section_name) 
        DO UPDATE SET content = EXCLUDED.content, updated_at = NOW()
        "#
    )
    .bind(&content_json)
    .execute(&db.pool)
    .await?;

    Ok(HomepageHero {
        title: body.title,
        subtitle: body.subtitle,
        cta_text: body.cta_text,
        cta_link: body.cta_link,
    })
}