use axum::{extract::{Path, State}, Json};
use serde_json::json;

use crate::config::db::DB;
use crate::core::errors::AppError;

use super::schema::{UpdateRoleBody};
use super::service;

pub async fn list_users(State(db): State<DB>) -> Result<Json<serde_json::Value>, AppError> {
    let items = service::list_users(&db).await?;
    Ok(Json(json!({ "ok": true, "data": items })))
}

pub async fn update_role(
    State(db): State<DB>,
    Path(id): Path<i32>,
    Json(body): Json<UpdateRoleBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    let out = service::update_role(&db, id, body.role).await?;
    Ok(Json(json!({ "ok": true, "data": out })))
}
