use axum::{routing::post, Router};
use super::controller;
use crate::config::{db::DB, env::Env};

pub fn routes(_db: DB, _env: Env) -> Router {
    // Note: DB และ Env ถูก inject ผ่าน Extension ใน main router แล้ว
    // แต่รับเข้ามาใน parameter เพื่อให้ signature ตรงกับรูปแบบทั่วไป
    Router::new()
        .route("/register", post(controller::register))
        .route("/login", post(controller::login))
}