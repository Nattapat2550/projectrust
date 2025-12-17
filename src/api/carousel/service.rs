use sqlx::Row;

use crate::config::db::DB;
use crate::core::errors::AppError;

use super::schema::{CarouselItem, CreateCarouselBody, UpdateCarouselBody};

pub async fn list(db: &DB) -> Result<Vec<CarouselItem>, AppError> {
    let rows = sqlx::query(
        r#"
        SELECT id, image_url, title, subtitle, link
        FROM carousel_items
        ORDER BY id DESC
        "#,
    )
    .fetch_all(&db.pool)
    .await?;

    let mut out = Vec::with_capacity(rows.len());
    for r in rows {
        out.push(CarouselItem {
            id: r.get("id"),
            image_url: r.get("image_url"),
            title: r.get("title"),
            subtitle: r.get("subtitle"),
            link: r.get("link"),
        });
    }
    Ok(out)
}

pub async fn create(db: &DB, body: CreateCarouselBody) -> Result<CarouselItem, AppError> {
    if body.image_url.trim().is_empty() {
        return Err(AppError::bad_request("image_url is required"));
    }
    if body.title.trim().is_empty() {
        return Err(AppError::bad_request("title is required"));
    }

    let row = sqlx::query(
        r#"
        INSERT INTO carousel_items (image_url, title, subtitle, link)
        VALUES ($1, $2, $3, $4)
        RETURNING id, image_url, title, subtitle, link
        "#,
    )
    .bind(body.image_url.trim())
    .bind(body.title.trim())
    .bind(body.subtitle.trim())
    .bind(body.link.trim())
    .fetch_one(&db.pool)
    .await?;

    Ok(CarouselItem {
        id: row.get("id"),
        image_url: row.get("image_url"),
        title: row.get("title"),
        subtitle: row.get("subtitle"),
        link: row.get("link"),
    })
}

pub async fn update(db: &DB, id: i32, body: UpdateCarouselBody) -> Result<CarouselItem, AppError> {
    let existing = sqlx::query(
        r#"
        SELECT id, image_url, title, subtitle, link
        FROM carousel_items
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&db.pool)
    .await?;

    let Some(existing) = existing else {
        return Err(AppError::not_found("CAROUSEL_NOT_FOUND", "Carousel item not found"));
    };

    let mut image_url: String = existing.get("image_url");
    let mut title: String = existing.get("title");
    let mut subtitle: String = existing.get("subtitle");
    let mut link: String = existing.get("link");

    if let Some(v) = body.image_url {
        if !v.trim().is_empty() { image_url = v.trim().to_string(); }
    }
    if let Some(v) = body.title {
        if !v.trim().is_empty() { title = v.trim().to_string(); }
    }
    if let Some(v) = body.subtitle {
        subtitle = v.trim().to_string();
    }
    if let Some(v) = body.link {
        link = v.trim().to_string();
    }

    let row = sqlx::query(
        r#"
        UPDATE carousel_items
        SET image_url=$2, title=$3, subtitle=$4, link=$5
        WHERE id=$1
        RETURNING id, image_url, title, subtitle, link
        "#,
    )
    .bind(id)
    .bind(&image_url)
    .bind(&title)
    .bind(&subtitle)
    .bind(&link)
    .fetch_one(&db.pool)
    .await?;

    Ok(CarouselItem {
        id: row.get("id"),
        image_url: row.get("image_url"),
        title: row.get("title"),
        subtitle: row.get("subtitle"),
        link: row.get("link"),
    })
}

pub async fn delete(db: &DB, id: i32) -> Result<(), AppError> {
    let res = sqlx::query(
        r#"
        DELETE FROM carousel_items
        WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(&db.pool)
    .await?;

    if res.rows_affected() == 0 {
        return Err(AppError::not_found("CAROUSEL_NOT_FOUND", "Carousel item not found"));
    }

    Ok(())
}
