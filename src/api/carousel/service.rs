use sqlx::Row;

use crate::config::db::DB;
use crate::core::errors::AppError;

use super::schema::{CarouselItem, CreateCarouselBody, UpdateCarouselBody};

pub async fn list(db: &DB) -> Result<Vec<CarouselItem>, AppError> {
    let rows = sqlx::query(
        r#"
        SELECT id, item_index, title, subtitle, description, image_dataurl, created_at, updated_at
        FROM carousel_items
        ORDER BY item_index ASC, id ASC
        "#,
    )
    .fetch_all(&db.pool)
    .await?;

    let mut out = Vec::with_capacity(rows.len());
    for r in rows {
        out.push(CarouselItem {
            id: r.get("id"),
            item_index: r.get("item_index"),
            title: r.try_get("title").ok(),
            subtitle: r.try_get("subtitle").ok(),
            description: r.try_get("description").ok(),
            image_dataurl: r.get("image_dataurl"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        });
    }
    Ok(out)
}

pub async fn create(db: &DB, body: CreateCarouselBody) -> Result<CarouselItem, AppError> {
    let idx = body.item_index.unwrap_or(0);

    let row = sqlx::query(
        r#"
        INSERT INTO carousel_items (item_index, title, subtitle, description, image_dataurl, created_at, updated_at)
        VALUES ($1,$2,$3,$4,$5,NOW(),NOW())
        RETURNING id, item_index, title, subtitle, description, image_dataurl, created_at, updated_at
        "#,
    )
    .bind(idx)
    .bind(body.title)
    .bind(body.subtitle)
    .bind(body.description)
    .bind(body.image_dataurl)
    .fetch_one(&db.pool)
    .await?;

    Ok(CarouselItem {
        id: row.get("id"),
        item_index: row.get("item_index"),
        title: row.try_get("title").ok(),
        subtitle: row.try_get("subtitle").ok(),
        description: row.try_get("description").ok(),
        image_dataurl: row.get("image_dataurl"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

pub async fn update(db: &DB, id: i32, body: UpdateCarouselBody) -> Result<CarouselItem, AppError> {
    let row = sqlx::query(
        r#"
        UPDATE carousel_items
        SET item_index = COALESCE($2, item_index),
            title = COALESCE($3, title),
            subtitle = COALESCE($4, subtitle),
            description = COALESCE($5, description),
            image_dataurl = COALESCE($6, image_dataurl),
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, item_index, title, subtitle, description, image_dataurl, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(body.item_index)
    .bind(body.title)
    .bind(body.subtitle)
    .bind(body.description)
    .bind(body.image_dataurl)
    .fetch_optional(&db.pool)
    .await?;

    let Some(row) = row else {
        return Err(AppError::not_found("CAROUSEL_NOT_FOUND", "Carousel item not found"));
    };

    Ok(CarouselItem {
        id: row.get("id"),
        item_index: row.get("item_index"),
        title: row.try_get("title").ok(),
        subtitle: row.try_get("subtitle").ok(),
        description: row.try_get("description").ok(),
        image_dataurl: row.get("image_dataurl"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

pub async fn delete(db: &DB, id: i32) -> Result<(), AppError> {
    let res = sqlx::query("DELETE FROM carousel_items WHERE id = $1")
        .bind(id)
        .execute(&db.pool)
        .await?;

    if res.rows_affected() == 0 {
        return Err(AppError::not_found("CAROUSEL_NOT_FOUND", "Carousel item not found"));
    }
    Ok(())
}
