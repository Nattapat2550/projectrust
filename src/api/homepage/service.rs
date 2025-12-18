use sqlx::Row;

use crate::config::db::DB;
use crate::core::errors::AppError;

use super::schema::{HomepageHero, HomepageHeroBody};

pub async fn get_hero(db: &DB) -> Result<HomepageHero, AppError> {
    let row = sqlx::query(
        r#"
        SELECT title, subtitle, cta_text, cta_link
        FROM homepage_hero
        ORDER BY id DESC
        LIMIT 1
        "#,
    )
    .fetch_optional(&db.pool)
    .await?;

    if let Some(r) = row {
        return Ok(HomepageHero {
            title: r.get("title"),
            subtitle: r.get("subtitle"),
            cta_text: r.get("cta_text"),
            cta_link: r.get("cta_link"),
        });
    }

    Ok(HomepageHero {
        title: "Welcome".into(),
        subtitle: "Pure API running".into(),
        cta_text: "Get Started".into(),
        cta_link: "/".into(),
    })
}

pub async fn put_hero(db: &DB, body: HomepageHeroBody) -> Result<HomepageHero, AppError> {
    if body.title.trim().is_empty() {
        return Err(AppError::bad_request("title is required"));
    }

    let row = sqlx::query(
        r#"
        INSERT INTO homepage_hero (title, subtitle, cta_text, cta_link)
        VALUES ($1, $2, $3, $4)
        RETURNING title, subtitle, cta_text, cta_link
        "#,
    )
    .bind(body.title.trim())
    .bind(body.subtitle.trim())
    .bind(body.cta_text.trim())
    .bind(body.cta_link.trim())
    .fetch_one(&db.pool)
    .await?;

    Ok(HomepageHero {
        title: row.get("title"),
        subtitle: row.get("subtitle"),
        cta_text: row.get("cta_text"),
        cta_link: row.get("cta_link"),
    })
}
