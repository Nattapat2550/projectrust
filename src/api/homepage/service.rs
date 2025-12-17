use crate::config::db::DB;
use crate::core::errors::AppError;
use super::schema::HomepageContent;

pub async fn get_content(db: &DB, section: &str) -> Result<HomepageContent, AppError> {
    let content = sqlx::query_as::<_, HomepageContent>(
        "SELECT section_name, content FROM homepage_content WHERE section_name = $1"
    )
    .bind(section)
    .fetch_optional(&db.pool)
    .await?
    .ok_or(AppError::NotFound(format!("Section '{}' not found", section)))?;

    Ok(content)
}

pub async fn update_content(db: &DB, section: &str, content: &str) -> Result<(), AppError> {
    sqlx::query(
        r#"
        INSERT INTO homepage_content (section_name, content, updated_at)
        VALUES ($1, $2, NOW())
        ON CONFLICT (section_name) 
        DO UPDATE SET content = $2, updated_at = NOW()
        "#
    )
    .bind(section)
    .bind(content)
    .execute(&db.pool)
    .await?;
    Ok(())
}