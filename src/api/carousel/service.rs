use crate::config::db::DB;
use crate::core::errors::AppError;
use super::schema::{CarouselItem, CreateCarouselPayload};

pub async fn get_all(db: &DB) -> Result<Vec<CarouselItem>, AppError> {
    let items = sqlx::query_as::<_, CarouselItem>(
        r#"
        SELECT id, item_index, title, subtitle, description, image_dataurl 
        FROM carousel_items 
        ORDER BY item_index ASC
        "#
    )
    .fetch_all(&db.pool)
    .await?;
    Ok(items)
}

pub async fn create(db: &DB, payload: CreateCarouselPayload) -> Result<(), AppError> {
    sqlx::query(
        r#"
        INSERT INTO carousel_items (item_index, title, subtitle, description, image_dataurl)
        VALUES ($1, $2, $3, $4, $5)
        "#
    )
    .bind(payload.item_index.unwrap_or(0))
    .bind(payload.title)
    .bind(payload.subtitle)
    .bind(payload.description)
    .bind(payload.image_dataurl)
    .execute(&db.pool)
    .await?;
    Ok(())
}