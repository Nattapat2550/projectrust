use axum::{extract::{Path, State}, Json};
use serde_json::json;

use crate::config::db::DB;
use crate::core::errors::AppError;

use super::schema::{CreateCarouselBody, UpdateCarouselBody};
use super::service;

pub async fn list(State(db): State<DB>) -> Result<Json<serde_json::Value>, AppError> {
    let items = service::list(&db).await?;
    Ok(Json(json!({ "ok": true, "data": items })))
}

pub async fn create(
    State(db): State<DB>,
    Json(body): Json<CreateCarouselBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    let out = service::create(&db, body).await?;
    Ok(Json(json!({ "ok": true, "data": out })))
}

pub async fn update(
    State(db): State<DB>,
    Path(id): Path<i32>,
    Json(body): Json<UpdateCarouselBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    let out = service::update(&db, id, body).await?;
    Ok(Json(json!({ "ok": true, "data": out })))
}

pub async fn delete(
    State(db): State<DB>,
    Path(id): Path<i32>,
) -> Result<Json<serde_json::Value>, AppError> {
    service::delete(&db, id).await?;
    Ok(Json(json!({ "ok": true })))
}
