use axum::{Extension, Json};
use serde_json::{json, Value};
use validator::Validate;
use crate::config::db::DB;
use crate::core::errors::AppError;
use super::service;
use super::schema::CreateCarouselPayload;

pub async fn get_carousels(Extension(db): Extension<DB>) -> Result<Json<Value>, AppError> {
    // แก้ไข: เรียก service::get_all
    let items = service::get_all(&db).await?;
    Ok(Json(json!(items)))
}

pub async fn create_carousel(
    Extension(db): Extension<DB>,
    Json(payload): Json<CreateCarouselPayload>,
) -> Result<Json<Value>, AppError> {
    // Validate Input
    if let Err(e) = payload.validate() {
        return Err(AppError::ValidationError(e.to_string()));
    }

    // แก้ไข: เรียก service::create
    service::create(&db, payload).await?;
    Ok(Json(json!({ "message": "Carousel item created successfully" })))
}