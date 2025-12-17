use axum::{extract::State, Json};
use serde_json::json;

use crate::config::db::DB;
use crate::core::errors::AppError;

use super::schema::{CreateClientBody, UpdateClientBody};
use super::service;

pub async fn list_clients(State(db): State<DB>) -> Result<Json<serde_json::Value>, AppError> {
    let items = service::list_clients(&db).await?;
    Ok(Json(json!({ "ok": true, "data": items })))
}

pub async fn create_client(
    State(db): State<DB>,
    Json(body): Json<CreateClientBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    let created = service::create_client(&db, body).await?;
    Ok(Json(json!({ "ok": true, "data": created })))
}

pub async fn update_client(
    State(db): State<DB>,
    axum::extract::Path(id): axum::extract::Path<i32>,
    Json(body): Json<UpdateClientBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    let updated = service::update_client(&db, id, body).await?;
    Ok(Json(json!({ "ok": true, "data": updated })))
}

pub async fn delete_client(
    State(db): State<DB>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> Result<Json<serde_json::Value>, AppError> {
    service::delete_client(&db, id).await?;
    Ok(Json(json!({ "ok": true })))
}
