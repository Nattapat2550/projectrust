use axum::{extract::Path, Extension, Json};
use serde_json::{json, Value};
use crate::config::db::DB;
use crate::core::errors::AppError;
use super::service;
use super::schema::UpdateContentPayload;

pub async fn get_section(
    Extension(db): Extension<DB>,
    Path(section_name): Path<String>,
) -> Result<Json<Value>, AppError> {
    let data = service::get_content(&db, &section_name).await?;
    Ok(Json(json!(data)))
}

pub async fn update_section(
    Extension(db): Extension<DB>,
    Path(section_name): Path<String>,
    Json(payload): Json<UpdateContentPayload>,
) -> Result<Json<Value>, AppError> {
    service::update_content(&db, &section_name, &payload.content).await?;
    Ok(Json(json!({ "message": "Content updated" })))
}