use axum::{
    extract::{Path, State},
    Extension,
    Json,
};
use serde_json::{json, Value};

use crate::config::{db::DB, env::Env};
use crate::core::errors::AppError;
use crate::core::middleware::jwt_auth::AuthUser;

use super::schema::{UpdateMeBody, UpdateRoleBody};
use super::service;

// Helper type alias
type AppState = State<(DB, Env)>;

// --------------------
// pure-api1 compatible
// --------------------

// GET /api/users/me
pub async fn get_me(
    State((db, _)): AppState,
    Extension(user): Extension<AuthUser>,
) -> Result<Json<Value>, AppError> {
    let u = service::get_by_id(&db, user.id).await?;
    Ok(Json(json!({ "ok": true, "data": u })))
}

// PATCH /api/users/me
pub async fn patch_me(
    State((db, _)): AppState,
    Extension(user): Extension<AuthUser>,
    Json(body): Json<UpdateMeBody>,
) -> Result<Json<Value>, AppError> {
    let u = service::update_me(&db, user.id, body).await?;
    Ok(Json(json!({ "ok": true, "data": u })))
}

// --------------------
// Admin (existing)
// --------------------

// GET /api/users
pub async fn list_users(State((db, _)): AppState) -> Result<Json<Value>, AppError> {
    let users = service::list_users(&db).await?;
    Ok(Json(json!({ "ok": true, "data": users })))
}

// PATCH /api/users/:id/role
pub async fn update_role(
    State((db, _)): AppState,
    Path(id): Path<i32>,
    Json(body): Json<UpdateRoleBody>,
) -> Result<Json<Value>, AppError> {
    let user = service::update_role(&db, id, body.role).await?;
    Ok(Json(json!({ "ok": true, "data": user })))
}
