use axum::{extract::{Path, State}, Json};
use serde_json::json;

use crate::config::db::DB;
use crate::core::errors::AppError;

use super::schema::*;
use super::service;

/// NOTE: กลุ่ม internal นี้ทำให้ behavior ใกล้ pure-api1 ที่มี internalRoutes
/// ทุก route ถูก wrap ด้วย apiKeyAuth ใน api/mod.rs แล้ว (internal อีกชั้น)

pub async fn find_user(State(db): State<DB>, Json(body): Json<FindUserBody>) -> Result<Json<serde_json::Value>, AppError> {
    let u = service::find_user(&db, body).await?;
    Ok(Json(json!({ "ok": true, "data": u })))
}

pub async fn get_verification_token(State(db): State<DB>, Path(email): Path<String>) -> Result<Json<serde_json::Value>, AppError> {
    let token = service::get_verification_token(&db, email).await?;
    Ok(Json(json!({ "ok": true, "data": { "token": token } })))
}

pub async fn get_reset_token(State(db): State<DB>, Path(email): Path<String>) -> Result<Json<serde_json::Value>, AppError> {
    let token = service::get_reset_token(&db, email).await?;
    Ok(Json(json!({ "ok": true, "data": { "token": token } })))
}

pub async fn list_users(State(db): State<DB>) -> Result<Json<serde_json::Value>, AppError> {
    let items = service::list_users(&db).await?;
    Ok(Json(json!({ "ok": true, "data": items })))
}

pub async fn list_clients(State(db): State<DB>) -> Result<Json<serde_json::Value>, AppError> {
    let items = service::list_clients(&db).await?;
    Ok(Json(json!({ "ok": true, "data": items })))
}

pub async fn set_client_active(
    State(db): State<DB>,
    Path(id): Path<i32>,
    Json(body): Json<SetClientActiveBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    service::set_client_active(&db, id, body.is_active).await?;
    Ok(Json(json!({ "ok": true })))
}

pub async fn get_homepage_hero(State(db): State<DB>) -> Result<Json<serde_json::Value>, AppError> {
    let hero = service::get_homepage_hero(&db).await?;
    Ok(Json(json!({ "ok": true, "data": hero })))
}

pub async fn put_homepage_hero(
    State(db): State<DB>,
    Json(body): Json<HomepageHeroBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    let out = service::put_homepage_hero(&db, body).await?;
    Ok(Json(json!({ "ok": true, "data": out })))
}

pub async fn get_carousel(State(db): State<DB>) -> Result<Json<serde_json::Value>, AppError> {
    let items = service::get_carousel(&db).await?;
    Ok(Json(json!({ "ok": true, "data": items })))
}

pub async fn create_carousel(
    State(db): State<DB>,
    Json(body): Json<CreateCarouselBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    let out = service::create_carousel(&db, body).await?;
    Ok(Json(json!({ "ok": true, "data": out })))
}

pub async fn update_carousel(
    State(db): State<DB>,
    Path(id): Path<i32>,
    Json(body): Json<UpdateCarouselBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    let out = service::update_carousel(&db, id, body).await?;
    Ok(Json(json!({ "ok": true, "data": out })))
}

pub async fn delete_carousel(
    State(db): State<DB>,
    Path(id): Path<i32>,
) -> Result<Json<serde_json::Value>, AppError> {
    service::delete_carousel(&db, id).await?;
    Ok(Json(json!({ "ok": true })))
}
