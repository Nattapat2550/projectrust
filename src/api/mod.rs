use axum::{
    extract::DefaultBodyLimit,
    http::{HeaderValue, Method, StatusCode, header},
    middleware,
    Extension, Json, Router,
};
use serde_json::json;
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    set_header::SetResponseHeaderLayer,
    trace::TraceLayer,
};
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use std::sync::Arc;
use std::time::Duration;

use crate::config::{db::DB, env::Env};
use crate::core::middleware::{api_key, jwt_auth};

pub mod admin;
pub mod auth;
pub mod carousel;
pub mod homepage;
pub mod internal;
pub mod root;
pub mod users;
pub mod download;

pub fn router(db: DB, env: Env) -> Router {
    // --- 1. Security Headers ---
    let security_headers = ServiceBuilder::new()
        .layer(SetResponseHeaderLayer::overriding(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::X_FRAME_OPTIONS,
            HeaderValue::from_static("SAMEORIGIN"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::X_XSS_PROTECTION,
            HeaderValue::from_static("1; mode=block"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::STRICT_TRANSPORT_SECURITY,
            HeaderValue::from_static("max-age=15552000; includeSubDomains"),
        ));

    // --- 2. CORS Setup ---
    let cors_layer = if env.allowed_origins.is_empty() {
        CorsLayer::permissive()
    } else {
        let origins: Vec<HeaderValue> = env
            .allowed_origins
            .iter()
            .map(|s| s.parse().unwrap())
            .collect();
        
        CorsLayer::new()
            .allow_origin(origins)
            .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::PATCH])
            .allow_headers(tower_http::cors::Any)
            .allow_credentials(false)
    };

    // --- 3. Rate Limiting for Auth ---
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .period(Duration::from_secs(30)) 
            .burst_size(env.rate_limit_auth_max as u32)
            .finish()
            .unwrap(),
    );

    // --- Routes Assembly ---

    // Auth Routes + Rate Limit
    let auth_routes = auth::routes::routes(db.clone(), env.clone())
        .layer(GovernorLayer {
            config: governor_conf.clone(),
        });

    // Protected User Routes (Admin)
    // ✅ แก้ไข: ส่ง env.clone() ไปด้วย และไม่ต้องใส่ .route_layer ซ้ำ เพราะใน users::routes ใส่ไว้แล้ว
    let users_routes = users::routes::routes(db.clone(), env.clone());

    // Protected Admin Routes
    let admin_routes = admin::routes::routes(db.clone())
        .route_layer(middleware::from_fn(jwt_auth::mw_jwt_auth));

    // Data Routes
    let homepage_routes = homepage::routes::routes(db.clone());
    let carousel_routes = carousel::routes::routes(db.clone());
    let download_routes = download::routes::routes(env.clone());
    
    // Internal Routes
    let internal_routes = internal::routes::routes(db.clone());

    // API Group
    let api_routes = Router::new()
        .nest("/auth", auth_routes)
        .nest("/users", users_routes)
        .nest("/homepage", homepage_routes)
        .nest("/carousel", carousel_routes)
        .nest("/download", download_routes)
        .nest("/admin", admin_routes)
        .nest("/internal", internal_routes)
        .layer(middleware::from_fn(api_key::mw_api_key_auth));

    let root_routes = root::routes::routes();

    // 404 Fallback Handler
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

    // --- Final Router ---
    Router::new()
        .merge(root_routes)
        .nest("/api", api_routes)
        .fallback(fallback_handler)
        // Global Layers
        .layer(DefaultBodyLimit::max(2 * 1024 * 1024))
        .layer(cors_layer)
        .layer(security_headers)
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .layer(Extension(db))
        .layer(Extension(env))
}