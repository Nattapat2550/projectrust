use axum::http::{HeaderValue, Method};
use tower_http::cors::{Any, CorsLayer};

use crate::config::env::Env;

pub fn cors_layer(env: &Env) -> CorsLayer {
    // pure-api1 มัก allow origins จาก env
    // ถ้า env ไม่ตั้ง จะ allow any ใน dev (เพื่อให้ไม่พัง)
    let mut layer = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::PATCH, Method::DELETE, Method::OPTIONS])
        .allow_headers(Any)
        .allow_credentials(true);

    if env.allowed_origins.is_empty() {
        layer = layer.allow_origin(Any);
    } else {
        let origins: Vec<HeaderValue> = env
            .allowed_origins
            .iter()
            .filter_map(|o| o.parse().ok())
            .collect();

        layer = layer.allow_origin(origins);
    }

    layer
}
