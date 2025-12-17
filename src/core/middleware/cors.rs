use std::time::Duration;

use axum::http::{header, HeaderValue, Method};
use tower_http::cors::{AllowOrigin, CorsLayer, Any};

use crate::config::env::Env;

pub fn cors_layer(env: &Env) -> CorsLayer {
    // Methods ที่อนุญาต
    let methods = [
        Method::GET,
        Method::POST,
        Method::PUT,
        Method::PATCH,
        Method::DELETE,
        Method::OPTIONS,
    ];

    // Headers ที่อนุญาต (ห้ามใช้ * ถ้าจะ allow_credentials(true))
    let headers = [
        header::AUTHORIZATION,
        header::CONTENT_TYPE,
        header::ACCEPT,
        header::ORIGIN,
        header::HeaderName::from_static("x-api-key"),
        header::HeaderName::from_static("x-requested-with"),
    ];

    // expose headers ที่ frontend อาจต้องอ่าน
    let expose = [header::CONTENT_LENGTH, header::CONTENT_TYPE];

    // ถ้ามี allow origins ใน env -> ใช้แบบ list + allow_credentials(true)
    let origin_values: Vec<HeaderValue> = env
        .allowed_origins
        .iter()
        .filter_map(|o| o.parse::<HeaderValue>().ok())
        .collect();

    if origin_values.is_empty() {
        // ไม่มีการตั้งค่า origins -> เปิดกว้าง แต่ต้องปิด credentials เพื่อไม่ผิดกฎ
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(methods)
            .allow_headers(headers)
            .expose_headers(expose)
            .allow_credentials(false)
            .max_age(Duration::from_secs(60 * 60))
    } else {
        CorsLayer::new()
            .allow_origin(AllowOrigin::list(origin_values))
            .allow_methods(methods)
            .allow_headers(headers) // ✅ สำคัญ: ไม่ใช้ Any
            .expose_headers(expose)
            .allow_credentials(true)
            .max_age(Duration::from_secs(60 * 60))
    }
}
