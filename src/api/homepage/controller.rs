use axum::{extract::State, Json};
use serde_json::json;

use crate::config::db::DB;
use crate::core::errors::AppError;

use super::schema::HomepageHeroBody;
use super::service;

pub async fn get_hero(State(db): State<DB>) -> Result<Json<serde_json::Value>, AppError> {
    let hero = service::get_hero(&db).await?;
    Ok(Json(json!({ "ok": true, "data": hero })))
}

pub async fn put_hero(
    State(db): State<DB>,
    Json(body): Json<HomepageHeroBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    let out = service::put_hero(&db, body).await?;
    Ok(Json(json!({ "ok": true, "data": out })))
}
