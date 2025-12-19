use sqlx::Row;
use crate::config::db::DB;
use crate::core::errors::AppError;
use super::schema::{CarouselItem, CreateCarouselBody, UpdateCarouselBody};

pub async fn list(db: &DB) -> Result<Vec<CarouselItem>, AppError> {
    // ✅ Select ให้ครบทุก field ตาม struct ใหม่
    let rows = sqlx::query(
        r#"
        SELECT id, item_index, image_dataurl, title, subtitle, description, created_at, updated_at
        FROM carousel_items
        ORDER BY item_index ASC, id ASC
        "#
    )
    .fetch_all(&db.pool)
    .await?;

    let mut out = Vec::with_capacity(rows.len());
    for r in rows {
        out.push(CarouselItem {
            id: r.get("id"),
            item_index: r.get("item_index"),
            image_dataurl: r.get("image_dataurl"), // Map DB column -> Struct field
            title: r.get("title"),
            subtitle: r.get("subtitle"),
            description: r.get("description"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        });
    }
    Ok(out)
}

pub async fn create(db: &DB, body: CreateCarouselBody) -> Result<CarouselItem, AppError> {
    // เช็คค่าว่าง
    if body.image_dataurl.trim().is_empty() {
        return Err(AppError::bad_request("image_dataurl is required"));
    }

    // ✅ Insert item_index และ image_dataurl
    let row = sqlx::query(
        r#"
        INSERT INTO carousel_items (item_index, image_dataurl, title, subtitle, description, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
        RETURNING id, item_index, image_dataurl, title, subtitle, description, created_at, updated_at
        "#
    )
    .bind(body.item_index.unwrap_or(0)) // default 0 ถ้าไม่ส่งมา
    .bind(body.image_dataurl.trim())
    .bind(body.title)
    .bind(body.subtitle)
    .bind(body.description)
    .fetch_one(&db.pool)
    .await?;

    Ok(CarouselItem {
        id: row.get("id"),
        item_index: row.get("item_index"),
        image_dataurl: row.get("image_dataurl"),
        title: row.get("title"),
        subtitle: row.get("subtitle"),
        description: row.get("description"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

pub async fn update(db: &DB, id: i32, body: UpdateCarouselBody) -> Result<CarouselItem, AppError> {
    let existing = sqlx::query(
        "SELECT id, item_index, image_dataurl, title, subtitle, description FROM carousel_items WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&db.pool)
    .await?;

    let Some(existing) = existing else {
        return Err(AppError::not_found("CAROUSEL_NOT_FOUND", "Item not found"));
    };

    // Prepare variables for update
    let mut item_index: i32 = existing.get("item_index");
    let mut image_dataurl: String = existing.get("image_dataurl");
    let mut title: Option<String> = existing.get("title");
    let mut subtitle: Option<String> = existing.get("subtitle");
    let mut description: Option<String> = existing.get("description");

    // Apply updates if present
    if let Some(v) = body.item_index { item_index = v; }
    if let Some(v) = body.image_dataurl { 
        if !v.trim().is_empty() { image_dataurl = v.trim().to_string(); } 
    }
    if body.title.is_some() { title = body.title; }
    if body.subtitle.is_some() { subtitle = body.subtitle; }
    if body.description.is_some() { description = body.description; }

    // ✅ Update query
    let row = sqlx::query(
        r#"
        UPDATE carousel_items
        SET item_index=$2, image_dataurl=$3, title=$4, subtitle=$5, description=$6, updated_at=NOW()
        WHERE id=$1
        RETURNING id, item_index, image_dataurl, title, subtitle, description, created_at, updated_at
        "#
    )
    .bind(id)
    .bind(item_index)
    .bind(image_dataurl)
    .bind(title)
    .bind(subtitle)
    .bind(description)
    .fetch_one(&db.pool)
    .await?;

    Ok(CarouselItem {
        id: row.get("id"),
        item_index: row.get("item_index"),
        image_dataurl: row.get("image_dataurl"),
        title: row.get("title"),
        subtitle: row.get("subtitle"),
        description: row.get("description"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

pub async fn delete(db: &DB, id: i32) -> Result<(), AppError> {
    let res = sqlx::query("DELETE FROM carousel_items WHERE id = $1").bind(id).execute(&db.pool).await?;
    if res.rows_affected() == 0 {
        return Err(AppError::not_found("CAROUSEL_NOT_FOUND", "Item not found"));
    }
    Ok(())
}