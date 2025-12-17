use sqlx::Row;

use crate::config::db::DB;
use crate::core::errors::AppError;

use super::schema::{ClientRow, CreateClientBody, UpdateClientBody};

pub async fn list_clients(db: &DB) -> Result<Vec<ClientRow>, AppError> {
    let rows = sqlx::query(
        r#"
        SELECT id, name, api_key, is_active, created_at
        FROM api_clients
        ORDER BY id DESC
        "#,
    )
    .fetch_all(&db.pool)
    .await?;

    let mut out = Vec::with_capacity(rows.len());
    for r in rows {
        out.push(ClientRow {
            id: r.get("id"),
            name: r.get("name"),
            api_key: r.get("api_key"),
            is_active: r.get("is_active"),
            created_at: r.try_get("created_at").ok(),
        });
    }

    Ok(out)
}

pub async fn create_client(db: &DB, body: CreateClientBody) -> Result<ClientRow, AppError> {
    if body.name.trim().is_empty() {
        return Err(AppError::bad_request("name is required"));
    }
    if body.api_key.trim().is_empty() {
        return Err(AppError::bad_request("api_key is required"));
    }

    let is_active = body.is_active.unwrap_or(true);

    let row = sqlx::query(
        r#"
        INSERT INTO api_clients (name, api_key, is_active)
        VALUES ($1, $2, $3)
        RETURNING id, name, api_key, is_active, created_at
        "#,
    )
    .bind(body.name.trim())
    .bind(body.api_key.trim())
    .bind(is_active)
    .fetch_one(&db.pool)
    .await?;

    Ok(ClientRow {
        id: row.get("id"),
        name: row.get("name"),
        api_key: row.get("api_key"),
        is_active: row.get("is_active"),
        created_at: row.try_get("created_at").ok(),
    })
}

pub async fn update_client(db: &DB, id: i32, body: UpdateClientBody) -> Result<ClientRow, AppError> {
    let existing = sqlx::query(
        r#"
        SELECT id, name, api_key, is_active, created_at
        FROM api_clients
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&db.pool)
    .await?;

    let Some(existing) = existing else {
        return Err(AppError::not_found("CLIENT_NOT_FOUND", "Client not found"));
    };

    let mut name: String = existing.get("name");
    let mut api_key: String = existing.get("api_key");
    let mut is_active: bool = existing.get("is_active");
    let created_at: Option<chrono::NaiveDateTime> = existing.try_get("created_at").ok();

    if let Some(n) = body.name {
        if !n.trim().is_empty() {
            name = n.trim().to_string();
        }
    }
    if let Some(k) = body.api_key {
        if !k.trim().is_empty() {
            api_key = k.trim().to_string();
        }
    }
    if let Some(a) = body.is_active {
        is_active = a;
    }

    let row = sqlx::query(
        r#"
        UPDATE api_clients
        SET name = $2, api_key = $3, is_active = $4
        WHERE id = $1
        RETURNING id, name, api_key, is_active, created_at
        "#,
    )
    .bind(id)
    .bind(&name)
    .bind(&api_key)
    .bind(is_active)
    .fetch_one(&db.pool)
    .await?;

    Ok(ClientRow {
        id: row.get("id"),
        name: row.get("name"),
        api_key: row.get("api_key"),
        is_active: row.get("is_active"),
        created_at: row.try_get("created_at").ok().or(created_at),
    })
}

pub async fn delete_client(db: &DB, id: i32) -> Result<(), AppError> {
    let res = sqlx::query("DELETE FROM api_clients WHERE id = $1")
        .bind(id)
        .execute(&db.pool)
        .await?;

    if res.rows_affected() == 0 {
        return Err(AppError::not_found("CLIENT_NOT_FOUND", "Client not found"));
    }

    Ok(())
}
