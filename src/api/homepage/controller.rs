use axum::{extract::{Path, State}, Json};
use serde_json::json;

use crate::config::db::DB;
use crate::core::errors::AppError;

use super::schema::{HomepageHeroBody, UpsertSectionBody};
use super::service;

// Backward-compat: GET /api/homepage/hero
pub async fn get_hero(State(db): State<DB>) -> Result<Json<serde_json::Value>, AppError> {
    let hero = service::get_hero(&db).await?;
    Ok(Json(json!({ "ok": true, "data": hero })))
}

// Backward-compat: PUT /api/homepage/hero
pub async fn put_hero(
    State(db): State<DB>,
    Json(body): Json<HomepageHeroBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    let out = service::put_hero(&db, body).await?;
    Ok(Json(json!({ "ok": true, "data": out })))
}

// pure-api1: GET /api/homepage/:section
pub async fn get_section(
    State(db): State<DB>,
    Path(section): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    let out = service::get_section(&db, &section).await?;
    Ok(Json(json!({ "ok": true, "data": out })))
}

// pure-api1: PUT /api/homepage/:section  body { content: string }
pub async fn put_section(
    State(db): State<DB>,
    Path(section): Path<String>,
    Json(body): Json<UpsertSectionBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    let out = service::upsert_section(&db, &section, body.content).await?;
    Ok(Json(json!({ "ok": true, "data": out })))
}
