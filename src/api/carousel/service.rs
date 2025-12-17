use crate::config::db::DB;
use crate::core::errors::AppError;
use super::schema::{CarouselItem, CreateCarouselPayload};

pub async fn get_active_carousels(db: &DB) -> Result<Vec<CarouselItem>, AppError> {
    let items = sqlx::query_as::<_, CarouselItem>(
        "SELECT id, image_url, link, is_active FROM carousel WHERE is_active = true ORDER BY id DESC"
    )
    .fetch_all(&db.pool)
    .await?;
    
    Ok(items)
}

pub async fn create_carousel(db: &DB, payload: CreateCarouselPayload) -> Result<(), AppError> {
    sqlx::query("INSERT INTO carousel (image_url, link, is_active) VALUES ($1, $2, true)")
        .bind(payload.image_url)
        .bind(payload.link)
        .execute(&db.pool)
        .await?;
        
    Ok(())
}