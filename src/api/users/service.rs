use sqlx::Row;

use crate::config::db::DB;
use crate::core::errors::AppError;

use super::schema::UserRow;

pub async fn list_users(db: &DB) -> Result<Vec<UserRow>, AppError> {
    let rows = sqlx::query(
        r#"
        SELECT id, email, name, role, provider, is_verified
        FROM users
        ORDER BY id DESC
        "#,
    )
    .fetch_all(&db.pool)
    .await?;

    let mut out = Vec::with_capacity(rows.len());
    for r in rows {
        out.push(UserRow {
            id: r.get("id"),
            email: r.get("email"),
            name: r.get("name"),
            role: r.get("role"),
            provider: r.get("provider"),
            is_verified: r.get("is_verified"),
        });
    }
    Ok(out)
}

pub async fn update_role(db: &DB, id: i32, role: String) -> Result<UserRow, AppError> {
    let role = role.trim().to_string();
    if role.is_empty() {
        return Err(AppError::bad_request("role is required"));
    }

    let row = sqlx::query(
        r#"
        UPDATE users
        SET role = $2
        WHERE id = $1
        RETURNING id, email, name, role, provider, is_verified
        "#,
    )
    .bind(id)
    .bind(&role)
    .fetch_optional(&db.pool)
    .await?;

    let Some(row) = row else {
        return Err(AppError::not_found("USER_NOT_FOUND", "User not found"));
    };

    Ok(UserRow {
        id: row.get("id"),
        email: row.get("email"),
        name: row.get("name"),
        role: row.get("role"),
        provider: row.get("provider"),
        is_verified: row.get("is_verified"),
    })
}
