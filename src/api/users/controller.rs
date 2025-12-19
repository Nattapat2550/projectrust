use axum::{
    extract::{Path, State},
    Json,
};
use serde_json::json;

use crate::core::errors::AppError;
use super::schema::UpdateRoleBody; // เรียกใช้ Struct ที่เพิ่งสร้าง
use super::service;

// GET /api/users
pub async fn list_users(
    State((db, _)): State<(crate::config::db::DB, crate::config::env::Env)>,
) -> Result<Json<serde_json::Value>, AppError> {
    let users = service::list_users(&db).await?;
    Ok(Json(json!({ "ok": true, "data": users })))
}

// PATCH /api/users/:id/role
pub async fn update_role(
    State((db, _)): State<(crate::config::db::DB, crate::config::env::Env)>,
    Path(id): Path<i32>,
    Json(body): Json<UpdateRoleBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user = service::update_role(&db, id, body.role).await?;
    Ok(Json(json!({ "ok": true, "data": user })))
}