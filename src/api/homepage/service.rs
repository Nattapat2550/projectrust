use sqlx::Row;
use serde_json::json;

use crate::config::db::DB;
use crate::core::errors::AppError;

use super::schema::{HomepageHero, HomepageHeroBody, HomepageSectionRow};

/// ✅ Helper constant สำหรับ Default Hero
fn default_hero() -> HomepageHero {
    HomepageHero {
        title: "Welcome".into(),
        subtitle: "Pure API running".into(),
        cta_text: "Get Started".into(),
        cta_link: "/".into(),
    }
}

/// pure-api1: GET /api/homepage/:section
pub async fn get_section(db: &DB, section: &str) -> Result<HomepageSectionRow, AppError> {
    let row = sqlx::query(
        r#"
        SELECT section_name, content, updated_at
        FROM homepage_content
        WHERE section_name = $1
        "#,
    )
    .bind(section)
    .fetch_optional(&db.pool)
    .await?;

    match row {
        Some(r) => Ok(HomepageSectionRow {
            section_name: r.get("section_name"),
            content: r.get("content"),
            updated_at: r.get("updated_at"),
        }),
        None => Err(AppError::not_found("SECTION_NOT_FOUND", "Section not found")),
    }
}

/// pure-api1: PUT /api/homepage/:section  body { content: string }
pub async fn upsert_section(db: &DB, section: &str, content: String) -> Result<HomepageSectionRow, AppError> {
    let row = sqlx::query(
        r#"
        INSERT INTO homepage_content(section_name, content, updated_at)
        VALUES ($1, $2, NOW())
        ON CONFLICT (section_name)
        DO UPDATE SET content = EXCLUDED.content, updated_at = NOW()
        RETURNING section_name, content, updated_at
        "#,
    )
    .bind(section)
    .bind(content)
    .fetch_one(&db.pool)
    .await?;

    Ok(HomepageSectionRow {
        section_name: row.get("section_name"),
        content: row.get("content"),
        updated_at: row.get("updated_at"),
    })
}

// ----------------------
// Backward-compat (/hero)
// ----------------------

pub async fn get_hero(db: &DB) -> Result<HomepageHero, AppError> {
    let row = sqlx::query(
        r#"
        SELECT content
        FROM homepage_content
        WHERE section_name = 'hero'
        "#,
    )
    .fetch_optional(&db.pool)
    .await?;

    if let Some(r) = row {
        let content: String = r.get("content");
        let parsed: serde_json::Value =
            serde_json::from_str(&content).unwrap_or_else(|_| json!({}));
        // try parse as HomepageHero; fallback default
        let hero: Result<HomepageHero, _> = serde_json::from_value(parsed);
        Ok(hero.unwrap_or_else(|_| default_hero()))
    } else {
        Ok(default_hero())
    }
}

pub async fn put_hero(db: &DB, body: HomepageHeroBody) -> Result<HomepageHero, AppError> {
    let content_json = serde_json::to_string(&json!({
        "title": body.title,
        "subtitle": body.subtitle,
        "cta_text": body.cta_text,
        "cta_link": body.cta_link,
    }))
    .map_err(|_| AppError::new(axum::http::StatusCode::BAD_REQUEST, "BAD_REQUEST", "Invalid content"))?;

    sqlx::query(
        r#"
        INSERT INTO homepage_content(section_name, content, updated_at)
        VALUES ('hero', $1, NOW())
        ON CONFLICT (section_name) 
        DO UPDATE SET content = EXCLUDED.content, updated_at = NOW()
        "#,
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
