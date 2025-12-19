use axum::{
    extract::{Path, State},
    Json,
};
use serde_json::{json, Value};

use crate::core::errors::AppError;
use crate::config::{db::DB, env::Env};
use super::schema::UpdateRoleBody;
use super::service;

// Helper type alias ไม่จำเป็นต้องแก้ structure หลัก แต่ช่วยให้อ่านง่ายขึ้น
type AppState = State<(DB, Env)>;

// GET /api/users
pub async fn list_users(
    State((db, _)): AppState,
) -> Result<Json<Value>, AppError> {
    let users = service::list_users(&db).await?;
    Ok(Json(json!({ "ok": true, "data": users })))
}

// PATCH /api/users/:id/role
pub async fn update_role(
    State((db, _)): AppState,
    Path(id): Path<i32>,
    Json(body): Json<UpdateRoleBody>,
) -> Result<Json<Value>, AppError> {
    // เพิ่มการตรวจสอบเบื้องต้น (Validation) ถ้าจำเป็น
    // เช่น if body.role.is_empty() { return Err(AppError::bad_request("Role cannot be empty")); }
    
    let user = service::update_role(&db, id, body.role).await?;
    Ok(Json(json!({ "ok": true, "data": user })))
}