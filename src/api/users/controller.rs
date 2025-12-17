use axum::{extract::Path, Extension, Json};
use serde_json::{json, Value};
use crate::config::db::DB;
use crate::core::errors::AppError;
use super::service;

pub async fn get_users(Extension(db): Extension<DB>) -> Result<Json<Value>, AppError> {
    let users = service::get_all_users(&db).await?;
    Ok(Json(json!(users)))
}

pub async fn get_user(
    Extension(db): Extension<DB>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, AppError> {
    let user = service::get_user_by_id(&db, id).await?;
    Ok(Json(json!(user)))
}

pub async fn delete_user(
    Extension(db): Extension<DB>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, AppError> {
    service::delete_user(&db, id).await?;
    Ok(Json(json!({ "message": "User deleted successfully" })))
}