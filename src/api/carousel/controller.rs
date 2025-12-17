use axum::{Extension, Json};
use serde_json::{json, Value};
use validator::Validate;
use crate::config::db::DB;
use crate::core::errors::AppError;
use super::service;
use super::schema::CreateCarouselPayload;

pub async fn get_carousels(Extension(db): Extension<DB>) -> Result<Json<Value>, AppError> {
    let items = service::get_active_carousels(&db).await?;
    Ok(Json(json!(items)))
}

pub async fn create_carousel(
    Extension(db): Extension<DB>,
    Json(payload): Json<CreateCarouselPayload>,
) -> Result<Json<Value>, AppError> {
    if let Err(e) = payload.validate() {
        return Err(AppError::ValidationError(e.to_string()));
    }

    service::create_carousel(&db, payload).await?;
    Ok(Json(json!({ "message": "Carousel created successfully" })))
}