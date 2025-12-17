use crate::config::db::DB;
use crate::core::errors::AppError;
use super::schema::UserDto;

pub async fn get_all_users(db: &DB) -> Result<Vec<UserDto>, AppError> {
    let users = sqlx::query_as::<_, UserDto>(
        "SELECT id, username, role, created_at FROM users ORDER BY id ASC"
    )
    .fetch_all(&db.pool)
    .await?;

    Ok(users)
}

pub async fn get_user_by_id(db: &DB, id: i32) -> Result<UserDto, AppError> {
    let user = sqlx::query_as::<_, UserDto>(
        "SELECT id, username, role, created_at FROM users WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&db.pool)
    .await?
    .ok_or(AppError::NotFound(format!("User ID {} not found", id)))?;

    Ok(user)
}

pub async fn delete_user(db: &DB, id: i32) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(&db.pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("User ID {} not found", id)));
    }

    Ok(())
}