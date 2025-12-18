use axum::{
    extract::DefaultBodyLimit,
    http::{header, HeaderValue, Method, StatusCode},
    middleware,
    Extension, Json, Router,
};
use serde_json::json;
use std::{sync::Arc, time::Duration};
use tower::ServiceBuilder;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    set_header::SetResponseHeaderLayer,
    trace::TraceLayer,
};

use crate::config::{db::DB, env::Env};
use crate::core::middleware::api_key;

pub mod admin;
pub mod auth;
pub mod carousel;
pub mod download;
pub mod homepage;
pub mod internal;
pub mod root;
pub mod users;

/// รวมทุก route ของ pure-api (Rust) ให้ behavior ใกล้ pure-api1
/// - / และ /health เป็น public
/// - /api/* ถูกบังคับใช้ API Key (x-api-key) ทุก request
/// - /api/auth/* มี rate limit ตาม env.rate_limit_auth_max
/// - /api/carousel GET เป็น public, ส่วนแก้ไขต้อง JWT + admin
pub fn router(db: DB, env: Env) -> Router {
    // -----------------------------
    // 1) Security headers
    // -----------------------------
    let security_headers = ServiceBuilder::new()
        .layer(SetResponseHeaderLayer::overriding(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::X_FRAME_OPTIONS,
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::REFERRER_POLICY,
            HeaderValue::from_static("no-referrer"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::CONTENT_SECURITY_POLICY,
            HeaderValue::from_static("default-src 'none'; frame-ancestors 'none'"),
        ));

    // -----------------------------
    // 2) CORS (ใช้ allowed_origins จาก Env)
    // -----------------------------
    let cors_layer = if env.allowed_origins.is_empty() {
        CorsLayer::permissive()
    } else {
        let origins: Vec<HeaderValue> = env
            .allowed_origins
            .iter()
            .filter_map(|s| s.parse::<HeaderValue>().ok())
            .collect();

        CorsLayer::new()
            .allow_origin(origins)
            .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::PATCH])
            .allow_headers(tower_http::cors::Any)
            .allow_credentials(false)
    };

    // -----------------------------
    // 3) Rate limit เฉพาะ auth routes
    // -----------------------------
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .period(Duration::from_secs(30))
            .burst_size(env.rate_limit_auth_max as u32)
            .finish()
            .expect("invalid governor config"),
    );

    // -----------------------------
    // 4) Routes
    // -----------------------------
    let root_routes = root::routes::routes();

    let auth_routes = auth::routes::routes(db.clone(), env.clone()).layer(GovernorLayer {
        config: governor_conf,
    });

    let api_routes = Router::new()
        .nest("/auth", auth_routes)
        .nest("/users", users::routes::routes(db.clone()))
        .nest("/admin", admin::routes::routes(db.clone()))
        .nest("/download", download::routes::routes(env.clone()))
        .nest("/homepage", homepage::routes::routes(db.clone()))
        .nest("/carousel", carousel::routes::routes(db.clone()))
        .nest("/internal", internal::routes::routes(db.clone()))
        // ✅ บังคับ API Key ทั้ง /api
        .layer(middleware::from_fn(api_key::mw_api_key_auth))
        // middleware api_key ใช้ Extension<DB> จึงต้องมี 2 บรรทัดนี้
        .layer(Extension(db.clone()))
        .layer(Extension(env.clone()));

    // -----------------------------
    // 5) Fallback 404
    // -----------------------------
    let fallback_handler = || async {
        (
            StatusCode::NOT_FOUND,
            Json(json!({
                "error": {
                    "code": 404,
                    "message": "Route not found",
                    "type": "NOT_FOUND"
                }
            })),
        )
    };

    // -----------------------------
    // 6) Final app router
    // -----------------------------
    Router::new()
        .merge(root_routes)
        .nest("/api", api_routes)
        .fallback(fallback_handler)
        // ✅ ให้ตรง pure-api1: express.json({ limit: "2mb" })
        .layer(DefaultBodyLimit::max(2 * 1024 * 1024))
        .layer(cors_layer)
        .layer(security_headers)
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
}
