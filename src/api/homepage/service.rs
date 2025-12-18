use sqlx::Row;

use chrono::{DateTime, Utc};

use crate::config::db::DB;
use crate::core::errors::AppError;

use super::schema::{HomepageContentRow};

pub async fn get_section(db: &DB, section: &str) -> Result<HomepageContentRow, AppError> {
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

    let Some(r) = row else {
        return Err(AppError::not_found("SECTION_NOT_FOUND", "Section not found"));
    };

    let updated_at: DateTime<Utc> = r.get("updated_at");

    Ok(HomepageContentRow {
        section_name: r.get("section_name"),
        content: r.get("content"),
        updated_at: updated_at.to_rfc3339(),
    })
}

pub async fn upsert_section(db: &DB, section: &str, content: &str) -> Result<HomepageContentRow, AppError> {
    let row = sqlx::query(
        r#"
        INSERT INTO homepage_content (section_name, content, updated_at)
        VALUES ($1,$2,NOW())
        ON CONFLICT (section_name)
        DO UPDATE SET content = EXCLUDED.content, updated_at = NOW()
        RETURNING section_name, content, updated_at
        "#,
    )
    .bind(section)
    .bind(content)
    .fetch_one(&db.pool)
    .await?;

    let updated_at: DateTime<Utc> = row.get("updated_at");

    Ok(HomepageContentRow {
        section_name: row.get("section_name"),
        content: row.get("content"),
        updated_at: updated_at.to_rfc3339(),
    })
}
