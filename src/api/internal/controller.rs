use axum::{
    extract::{Path, State},
    Json,
};
use serde_json::json;

use crate::config::db::DB;
use crate::core::errors::AppError;

use super::schema::*;
use super::service;

/// Internal API response format (same as pure-api1):
/// { ok: true, data?: ... }
fn ok<T: serde::Serialize>(data: T) -> Json<serde_json::Value> {
    Json(json!({ "ok": true, "data": data }))
}
fn ok_empty() -> Json<serde_json::Value> {
    Json(json!({ "ok": true }))
}

// -------------------- Auth / Users --------------------

pub async fn find_user(
    State(db): State<DB>,
    Json(body): Json<FindUserBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user = service::find_user(&db, body).await?;
    Ok(ok(user))
}

pub async fn create_user_email(
    State(db): State<DB>,
    Json(body): Json<CreateUserEmailBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user = service::create_user_email(&db, body).await?;
    Ok(ok(user))
}

pub async fn set_oauth_user(
    State(db): State<DB>,
    Json(body): Json<SetOAuthUserBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user = service::set_oauth_user(&db, body).await?;
    Ok(ok(user))
}

pub async fn set_username_password(
    State(db): State<DB>,
    Json(body): Json<SetUsernamePasswordBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user = service::set_username_password(&db, body).await?;
    Ok(ok(user))
}

/// ✅ ใช้ตอน admin update username / profile picture (projectdocker)
pub async fn update_user(
    State(db): State<DB>,
    Json(body): Json<UpdateUserBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user = service::update_user(&db, body).await?;
    Ok(ok(user))
}

pub async fn list_users(State(db): State<DB>) -> Result<Json<serde_json::Value>, AppError> {
    let users = service::list_users(&db).await?;
    Ok(ok(users))
}

// -------------------- Verification / Reset --------------------

pub async fn store_verification_code(
    State(db): State<DB>,
    Json(body): Json<StoreVerificationCodeBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    service::store_verification_code(&db, body).await?;
    Ok(ok_empty())
}

pub async fn verify_code(
    State(db): State<DB>,
    Json(body): Json<VerifyCodeBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    let out = service::verify_code(&db, body).await?;
    Ok(ok(out))
}

pub async fn create_reset_token(
    State(db): State<DB>,
    Json(body): Json<CreateResetTokenBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    service::create_reset_token(&db, body).await?;
    Ok(ok_empty())
}

pub async fn consume_reset_token(
    State(db): State<DB>,
    Json(body): Json<ConsumeResetTokenBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    let ok_flag = service::consume_reset_token(&db, body).await?;
    Ok(ok(json!({ "ok": ok_flag })))
}

pub async fn set_password(
    State(db): State<DB>,
    Json(body): Json<SetPasswordBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    service::set_password(&db, body).await?;
    Ok(ok_empty())
}

// -------------------- Admin Clients --------------------

pub async fn list_clients(State(db): State<DB>) -> Result<Json<serde_json::Value>, AppError> {
    let clients = service::list_clients(&db).await?;
    Ok(ok(clients))
}

pub async fn set_client_active(
    State(db): State<DB>,
    Path(id): Path<i32>,
    Json(body): Json<SetClientActiveBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    service::set_client_active(&db, id, body.is_active).await?;
    Ok(ok_empty())
}

// -------------------- Homepage (internal for projectdocker) --------------------

pub async fn homepage_list(State(db): State<DB>) -> Result<Json<serde_json::Value>, AppError> {
    let rows = service::homepage_list(&db).await?;
    Ok(ok(rows))
}

pub async fn homepage_update(
    State(db): State<DB>,
    Json(body): Json<HomepageUpdateBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    let row = service::homepage_update(&db, body).await?;
    Ok(ok(row))
}

// -------------------- Carousel (internal for projectdocker) --------------------

pub async fn carousel_list(State(db): State<DB>) -> Result<Json<serde_json::Value>, AppError> {
    let rows = service::carousel_list(&db).await?;
    Ok(ok(rows))
}

pub async fn carousel_create(
    State(db): State<DB>,
    Json(body): Json<CarouselCreateBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    let row = service::carousel_create(&db, body).await?;
    Ok(ok(row))
}

pub async fn carousel_update(
    State(db): State<DB>,
    Json(body): Json<CarouselUpdateBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    let row = service::carousel_update(&db, body).await?;
    Ok(ok(row))
}

pub async fn carousel_delete(
    State(db): State<DB>,
    Json(body): Json<IdBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    service::carousel_delete(&db, body.id).await?;
    Ok(ok_empty())
}

// -------------------- Legacy Homepage Hero --------------------

pub async fn get_homepage_hero(State(db): State<DB>) -> Result<Json<serde_json::Value>, AppError> {
    let hero = service::get_homepage_hero(&db).await?;
    Ok(ok(hero))
}

pub async fn put_homepage_hero(
    State(db): State<DB>,
    Json(body): Json<HomepageHeroBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    let hero = service::put_homepage_hero(&db, body).await?;
    Ok(ok(hero))
}

// -------------------- Legacy Carousel --------------------

pub async fn get_carousel(State(db): State<DB>) -> Result<Json<serde_json::Value>, AppError> {
    let items = service::get_carousel(&db).await?;
    Ok(ok(items))
}

pub async fn create_carousel(
    State(db): State<DB>,
    Json(body): Json<CreateCarouselBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    let item = service::create_carousel(&db, body).await?;
    Ok(ok(item))
}

pub async fn update_carousel(
    State(db): State<DB>,
    Path(id): Path<i32>,
    Json(body): Json<UpdateCarouselBody>,
) -> Result<Json<serde_json::Value>, AppError> {
    let item = service::update_carousel(&db, id, body).await?;
    Ok(ok(item))
}

pub async fn delete_carousel(
    State(db): State<DB>,
    Path(id): Path<i32>,
) -> Result<Json<serde_json::Value>, AppError> {
    service::delete_carousel(&db, id).await?;
    Ok(ok_empty())
}

// -------------------- Debug Tokens (optional, keep) --------------------

pub async fn get_verification_token(
    State(db): State<DB>,
    Path(email): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    let token = service::get_verification_token(&db, email).await?;
    Ok(ok(token))
}

pub async fn get_reset_token(
    State(db): State<DB>,
    Path(email): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    let token = service::get_reset_token(&db, email).await?;
    Ok(ok(token))
}
