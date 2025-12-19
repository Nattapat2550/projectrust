use sqlx::Row;
use crate::config::db::DB;
use crate::core::errors::AppError;
use super::schema::{CarouselItem, CreateCarouselBody, UpdateCarouselBody};

pub async fn list(db: &DB) -> Result<Vec<CarouselItem>, AppError> {
    // ✅ แก้ SQL: ใช้ image_dataurl และ description ตาม db.sql
    let rows = sqlx::query(
        r#"
        SELECT id, image_dataurl, title, subtitle, description
        FROM carousel_items
        ORDER BY item_index ASC, id DESC
        "#
    )
    .fetch_all(&db.pool)
    .await?;

    let mut out = Vec::with_capacity(rows.len());
    for r in rows {
        out.push(CarouselItem {
            id: r.get("id"),
            image_url: r.get("image_dataurl"), // Map DB column -> Struct field
            title: r.get("title"),
            subtitle: r.get("subtitle"),
            description: r.get("description"),
        });
    }
    Ok(out)
}

pub async fn create(db: &DB, body: CreateCarouselBody) -> Result<CarouselItem, AppError> {
    if body.image_url.trim().is_empty() {
        return Err(AppError::bad_request("image_url is required"));
    }

    // ✅ แก้ SQL: Insert ลง image_dataurl
    let row = sqlx::query(
        r#"
        INSERT INTO carousel_items (image_dataurl, title, subtitle, description)
        VALUES ($1, $2, $3, $4)
        RETURNING id, image_dataurl, title, subtitle, description
        "#
    )
    .bind(body.image_url.trim())
    .bind(body.title)
    .bind(body.subtitle)
    .bind(body.description)
    .fetch_one(&db.pool)
    .await?;

    Ok(CarouselItem {
        id: row.get("id"),
        image_url: row.get("image_dataurl"),
        title: row.get("title"),
        subtitle: row.get("subtitle"),
        description: row.get("description"),
    })
}

pub async fn update(db: &DB, id: i32, body: UpdateCarouselBody) -> Result<CarouselItem, AppError> {
    let existing = sqlx::query(
        "SELECT id, image_dataurl, title, subtitle, description FROM carousel_items WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&db.pool)
    .await?;

    let Some(existing) = existing else {
        return Err(AppError::not_found("CAROUSEL_NOT_FOUND", "Item not found"));
    };

    let mut image_url: String = existing.get("image_dataurl");
    let mut title: Option<String> = existing.get("title");
    let mut subtitle: Option<String> = existing.get("subtitle");
    let mut description: Option<String> = existing.get("description");

    if let Some(v) = body.image_url { if !v.trim().is_empty() { image_url = v.trim().to_string(); } }
    if body.title.is_some() { title = body.title; }
    if body.subtitle.is_some() { subtitle = body.subtitle; }
    if body.description.is_some() { description = body.description; }

    let row = sqlx::query(
        r#"
        UPDATE carousel_items
        SET image_dataurl=$2, title=$3, subtitle=$4, description=$5, updated_at=NOW()
        WHERE id=$1
        RETURNING id, image_dataurl, title, subtitle, description
        "#
    )
    .bind(id)
    .bind(image_url)
    .bind(title)
    .bind(subtitle)
    .bind(description)
    .fetch_one(&db.pool)
    .await?;

    Ok(CarouselItem {
        id: row.get("id"),
        image_url: row.get("image_dataurl"),
        title: row.get("title"),
        subtitle: row.get("subtitle"),
        description: row.get("description"),
    })
}

pub async fn delete(db: &DB, id: i32) -> Result<(), AppError> {
    let res = sqlx::query("DELETE FROM carousel_items WHERE id = $1").bind(id).execute(&db.pool).await?;
    if res.rows_affected() == 0 {
        return Err(AppError::not_found("CAROUSEL_NOT_FOUND", "Item not found"));
    }
    Ok(())
}