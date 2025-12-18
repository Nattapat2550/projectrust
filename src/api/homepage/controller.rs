use axum::{extract::{Path, State}, Json};
use serde_json::json;

use crate::config::db::DB;
use crate::core::errors::AppError;

use super::schema::UpsertHomepageBody;
use super::service;

pub async fn get_section(
    State(db): State<DB>,
    Path(section): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    let out = service::get_section(&db, &section).await?;
    Ok(Json(json!({ "ok": true, "data": out })))
}

pub async fn put_section(
    State(db): State<DB>,
    Path(section): Path<String>,
    Json(body): Json<UpsertHomepageBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    let out = service::upsert_section(&db, &section, &body.content).await?;
    Ok(Json(json!({ "ok": true, "data": out })))
}
