use axum::{extract::{State, Path}, Json};
use serde_json::{json, Value}; // ✅ Import json!
use crate::config::db::DB;
use crate::core::errors::AppError;
use super::{service, schema::*};

// ✅ เปลี่ยน Return Type เป็น Json<Value> เพื่อรองรับ { ok: true, data: ... }
pub async fn find_user(State(db): State<DB>, Json(body): Json<FindUserBody>) -> Result<Json<Value>, AppError> {
    let user = service::find_user(&db, body).await?;
    // ✅ ถ้า None จะได้ data: null (ตามที่ Node.js ต้องการ)
    Ok(Json(json!({ "ok": true, "data": user })))
}

pub async fn create_user_email(State(db): State<DB>, Json(body): Json<CreateUserEmailBody>) -> Result<Json<Value>, AppError> {
    let user = service::create_user_email(&db, body).await?;
    Ok(Json(json!({ "ok": true, "data": user })))
}

pub async fn set_oauth_user(State(db): State<DB>, Json(body): Json<SetOAuthUserBody>) -> Result<Json<Value>, AppError> {
    let user = service::set_oauth_user(&db, body).await?;
    Ok(Json(json!({ "ok": true, "data": user })))
}

pub async fn store_verification_code(State(db): State<DB>, Json(body): Json<StoreVerificationCodeBody>) -> Result<Json<Value>, AppError> {
    service::store_verification_code(&db, body).await?;
    Ok(Json(json!({ "ok": true })))
}

// ⚠️ VerifyCodeResponse มี structure { ok: bool, ... } อยู่แล้ว จึงส่งตรงๆ
pub async fn verify_code(State(db): State<DB>, Json(body): Json<VerifyCodeBody>) -> Result<Json<VerifyCodeResponse>, AppError> {
    let res = service::verify_code(&db, body).await?;
    Ok(Json(res))
}

pub async fn set_username_password(State(db): State<DB>, Json(body): Json<SetUsernamePasswordBody>) -> Result<Json<Value>, AppError> {
    let user = service::set_username_password(&db, body).await?;
    Ok(Json(json!({ "ok": true, "data": user })))
}

pub async fn update_user(State(db): State<DB>, Json(body): Json<UpdateUserBody>) -> Result<Json<Value>, AppError> {
    let user = service::update_user(&db, body).await?;
    Ok(Json(json!({ "ok": true, "data": user })))
}

pub async fn create_reset_token(State(db): State<DB>, Json(body): Json<CreateResetTokenBody>) -> Result<Json<Value>, AppError> {
    service::create_reset_token(&db, body).await?;
    Ok(Json(json!({ "ok": true, "data": null })))
}

pub async fn consume_reset_token(State(db): State<DB>, Json(body): Json<ConsumeResetTokenBody>) -> Result<Json<Value>, AppError> {
    let user = service::consume_reset_token(&db, body).await?;
    Ok(Json(json!({ "ok": true, "data": user })))
}

pub async fn set_password(State(db): State<DB>, Json(body): Json<SetPasswordBody>) -> Result<Json<Value>, AppError> {
    service::set_password(&db, body).await?;
    Ok(Json(json!({ "ok": true, "data": null })))
}

// --- Controllers เดิม (Wrap ด้วย json! เพื่อความสม่ำเสมอ หรือตาม PureAPI) ---

pub async fn list_users(State(db): State<DB>) -> Result<Json<Value>, AppError> {
    let users = service::list_users(&db).await?;
    Ok(Json(json!({ "ok": true, "data": users })))
}

pub async fn list_clients(State(db): State<DB>) -> Result<Json<Value>, AppError> {
    let clients = service::list_clients(&db).await?;
    Ok(Json(json!({ "ok": true, "data": clients })))
}

pub async fn set_client_active(State(db): State<DB>, Path(id): Path<i32>, Json(body): Json<serde_json::Value>) -> Result<Json<Value>, AppError> {
    let active = body.get("is_active").and_then(|v| v.as_bool()).unwrap_or(true);
    service::set_client_active(&db, id, active).await?;
    Ok(Json(json!({ "ok": true })))
}

pub async fn get_homepage_hero(State(db): State<DB>) -> Result<Json<Value>, AppError> {
    let hero = service::get_homepage_hero(&db).await?;
    Ok(Json(json!({ "ok": true, "data": hero })))
}

pub async fn put_homepage_hero(State(db): State<DB>, Json(body): Json<HomepageHeroBody>) -> Result<Json<Value>, AppError> {
    let hero = service::put_homepage_hero(&db, body).await?;
    Ok(Json(json!({ "ok": true, "data": hero })))
}

pub async fn get_carousel(State(db): State<DB>) -> Result<Json<Value>, AppError> {
    let items = service::get_carousel(&db).await?;
    Ok(Json(json!({ "ok": true, "data": items })))
}

pub async fn create_carousel(State(db): State<DB>, Json(body): Json<CreateCarouselBody>) -> Result<Json<Value>, AppError> {
    let item = service::create_carousel(&db, body).await?;
    Ok(Json(json!({ "ok": true, "data": item })))
}

pub async fn update_carousel(State(db): State<DB>, Path(id): Path<i32>, Json(body): Json<UpdateCarouselBody>) -> Result<Json<Value>, AppError> {
    let item = service::update_carousel(&db, id, body).await?;
    Ok(Json(json!({ "ok": true, "data": item })))
}

pub async fn delete_carousel(State(db): State<DB>, Path(id): Path<i32>) -> Result<Json<Value>, AppError> {
    service::delete_carousel(&db, id).await?;
    Ok(Json(json!({ "ok": true })))
}

// Unused/Mock
pub async fn get_verification_token(State(db): State<DB>, Path(email): Path<String>) -> Result<Json<String>, AppError> {
    let token = service::get_verification_token(&db, email).await?;
    Ok(Json(token))
}
pub async fn get_reset_token(State(db): State<DB>, Path(email): Path<String>) -> Result<Json<String>, AppError> {
    let token = service::get_reset_token(&db, email).await?;
    Ok(Json(token))
}