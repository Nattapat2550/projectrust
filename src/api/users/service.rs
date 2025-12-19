use sqlx::Row;
use crate::config::db::DB;
use crate::core::errors::AppError;
use super::schema::UserRow;

pub async fn list_users(db: &DB) -> Result<Vec<UserRow>, AppError> {
    let rows = sqlx::query(
        // ✅ แก้ provider -> oauth_provider AS provider
        "SELECT id, email, username, role, oauth_provider AS provider, is_email_verified AS is_verified FROM users ORDER BY id DESC"
    )
    .fetch_all(&db.pool).await?;

    let mut out = Vec::new();
    for r in rows {
        out.push(UserRow {
            id: r.get("id"),
            email: r.get("email"),
            username: r.get("username"),
            role: r.get("role"),
            provider: r.get("provider"),
            is_verified: r.get("is_verified"),
        });
    }
    Ok(out)
}

pub async fn update_role(db: &DB, id: i32, role: String) -> Result<UserRow, AppError> {
    let row = sqlx::query(
        // ✅ แก้ provider -> oauth_provider AS provider ใน RETURNING
        "UPDATE users SET role = $2 WHERE id = $1 RETURNING id, email, username, role, oauth_provider AS provider, is_email_verified AS is_verified"
    )
    .bind(id).bind(&role).fetch_optional(&db.pool).await?;

    let Some(r) = row else { return Err(AppError::not_found("USER_NOT_FOUND", "User not found")); };

    Ok(UserRow {
        id: r.get("id"), email: r.get("email"), username: r.get("username"),
        role: r.get("role"), provider: r.get("provider"), is_verified: r.get("is_verified"),
    })
}