use axum::{extract::{Path, State}, Json};
use crate::config::db::DB;
use crate::core::errors::AppError;
use super::{service, schema::{UserRow, UpdateRoleBody}}; // ✅ Import UpdateRoleBody

pub async fn list_users(State(db): State<DB>) -> Result<Json<Vec<UserRow>>, AppError> {
    let users = service::list_users(&db).await?;
    Ok(Json(users))
}

pub async fn update_role(
    State(db): State<DB>,
    Path(id): Path<i32>,
    Json(body): Json<UpdateRoleBody>, // ✅ ใช้ Struct นี้
) -> Result<Json<UserRow>, AppError> {
    let user = service::update_role(&db, id, body.role).await?;
    Ok(Json(user))
}