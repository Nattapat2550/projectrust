use axum::{
    http::StatusCode,
    middleware,
    response::IntoResponse,
    Json,
    Router,
    Extension,
};
use serde_json::json;

use crate::config::{db::DB, env::Env};
use crate::core::middleware::{api_key, cors, jwt_auth};

pub mod auth;
pub mod users;
pub mod carousel;
pub mod homepage;
pub mod admin;
pub mod root;
pub mod download;
pub mod internal;

pub fn router(db: DB, env: Env) -> Router {
    // Public Routes (ไม่มี api key)
    let root_routes = root::routes::routes().fallback(not_found);

    // API Routes
    let auth_routes = auth::routes::routes(db.clone(), env.clone());
    let users_routes = users::routes::routes(db.clone());
    let homepage_routes = homepage::routes::routes(db.clone());
    let carousel_routes = carousel::routes::routes(db.clone());
    let download_routes = download::routes::routes(env.clone());

    // Protected Routes (Admin) — ต้อง JWT + admin
    let admin_routes = admin::routes::routes(db.clone())
        .route_layer(middleware::from_fn(jwt_auth::mw_require_admin))
        .route_layer(middleware::from_fn(jwt_auth::mw_jwt_auth));

    // Internal Routes — บังคับ API Key อีกชั้น (เหมือน pure-api1 ที่ internalRoutes.use(apiKeyAuth))
    let internal_routes = internal::routes::routes(db.clone())
        .route_layer(middleware::from_fn(api_key::mw_api_key_auth));

    let api_routes = Router::new()
        .nest("/auth", auth_routes)
        .nest("/users", users_routes)
        .nest("/homepage", homepage_routes)
        .nest("/carousel", carousel_routes)
        .nest("/download", download_routes)
        .nest("/admin", admin_routes)
        .nest("/internal", internal_routes)
        // บังคับทุก /api ต้องมี x-api-key (เหมือน pure-api1: app.use("/api", apiKeyAuth))
        .route_layer(middleware::from_fn(api_key::mw_api_key_auth))
        .fallback(not_found);

    let cors_layer = cors::cors_layer(&env);

    Router::new()
        .nest("/", root_routes)
        .nest("/api", api_routes)
        .layer(cors_layer)
        .layer(Extension(db))
        .layer(Extension(env))
        .fallback(not_found)
}

async fn not_found() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(json!({
            "ok": false,
            "error": { "code": "NOT_FOUND", "message": "Route not found" }
        })),
    )
}
